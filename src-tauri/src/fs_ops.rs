//! 文件系统操作：扫描 .md 文件、备份、还原。
//! 备份目录建在目标目录的同级：`<目标名>-backup-<时间戳>`，内含 manifest.json。

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 备份清单：记录原始根目录与被备份文件的相对路径（用 `/` 分隔，跨平台）。
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub original_root: String,
    pub created_at: String,
    pub files: Vec<String>,
}

/// 收集目录下的 .md 文件。`recursive == false` 时只看顶层。
pub fn collect_md_files(root: &Path, recursive: bool) -> Vec<PathBuf> {
    let max_depth = if recursive { usize::MAX } else { 1 };
    WalkDir::new(root)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .map(|x| x.eq_ignore_ascii_case("md"))
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// 把相对路径（以 `/` 分隔）安全地拼到 base 上，逐段 join 以兼容 Windows。
fn join_rel(base: &Path, rel: &str) -> PathBuf {
    rel.split('/').fold(base.to_path_buf(), |acc, c| acc.join(c))
}

/// 把 `files`（绝对路径，均在 `root` 之下）复制到一个新建的备份目录，写入 manifest，返回备份目录。
pub fn create_backup(root: &Path, files: &[PathBuf], timestamp: &str) -> Result<PathBuf, String> {
    let parent = root
        .parent()
        .ok_or_else(|| "无法确定目标目录的父目录".to_string())?;
    let name = root
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "无法确定目标目录名".to_string())?;
    let backup_dir = parent.join(format!("{name}-backup-{timestamp}"));
    fs::create_dir_all(&backup_dir).map_err(|e| format!("创建备份目录失败：{e}"))?;

    let mut rels = Vec::new();
    for f in files {
        let rel = f
            .strip_prefix(root)
            .map_err(|_| format!("文件不在目标目录内：{}", f.display()))?;
        let dest = backup_dir.join(rel);
        if let Some(p) = dest.parent() {
            fs::create_dir_all(p).map_err(|e| format!("创建备份子目录失败：{e}"))?;
        }
        fs::copy(f, &dest).map_err(|e| format!("备份文件失败 {}：{e}", f.display()))?;
        rels.push(rel.to_string_lossy().replace('\\', "/"));
    }

    let manifest = Manifest {
        original_root: root.to_string_lossy().to_string(),
        created_at: timestamp.to_string(),
        files: rels,
    };
    let json = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
    fs::write(backup_dir.join("manifest.json"), json)
        .map_err(|e| format!("写入 manifest 失败：{e}"))?;
    Ok(backup_dir)
}

/// 在目标目录同级查找最新的备份目录（时间戳格式可按字典序排序）。
pub fn find_latest_backup(root: &Path) -> Result<PathBuf, String> {
    let parent = root
        .parent()
        .ok_or_else(|| "无法确定目标目录的父目录".to_string())?;
    let name = root
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "无法确定目标目录名".to_string())?;
    let prefix = format!("{name}-backup-");

    let mut candidates: Vec<PathBuf> = fs::read_dir(parent)
        .map_err(|e| format!("读取父目录失败：{e}"))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .filter(|p| {
            p.file_name()
                .and_then(|s| s.to_str())
                .map(|n| n.starts_with(&prefix))
                .unwrap_or(false)
        })
        .collect();
    candidates.sort();
    candidates
        .pop()
        .ok_or_else(|| "未找到该目录的备份".to_string())
}

/// 从备份目录按 manifest 把文件拷回原位，返回 (备份目录路径, 还原文件数)。
pub fn restore_from_backup(backup_dir: &Path) -> Result<(String, usize), String> {
    let manifest_path = backup_dir.join("manifest.json");
    let raw = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("读取 manifest 失败：{e}"))?;
    let manifest: Manifest =
        serde_json::from_str(&raw).map_err(|e| format!("解析 manifest 失败：{e}"))?;
    let root = PathBuf::from(&manifest.original_root);

    let mut count = 0;
    for rel in &manifest.files {
        let src = join_rel(backup_dir, rel);
        let dest = join_rel(&root, rel);
        if let Some(p) = dest.parent() {
            fs::create_dir_all(p).map_err(|e| format!("创建目录失败：{e}"))?;
        }
        fs::copy(&src, &dest).map_err(|e| format!("还原文件失败 {}：{e}", dest.display()))?;
        count += 1;
    }
    Ok((backup_dir.to_string_lossy().to_string(), count))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn collect_respects_recursive_flag() {
        let tmp = tempdir().unwrap();
        let root = tmp.path().join("docs");
        fs::create_dir_all(root.join("sub")).unwrap();
        fs::write(root.join("a.md"), "a").unwrap();
        fs::write(root.join("b.txt"), "b").unwrap();
        fs::write(root.join("sub/c.md"), "c").unwrap();

        let top = collect_md_files(&root, false);
        assert_eq!(top.len(), 1); // 只有顶层 a.md

        let all = collect_md_files(&root, true);
        assert_eq!(all.len(), 2); // a.md + sub/c.md
    }

    #[test]
    fn backup_then_restore_roundtrip() {
        let tmp = tempdir().unwrap();
        let root = tmp.path().join("docs");
        fs::create_dir_all(root.join("sub")).unwrap();
        fs::write(root.join("a.md"), "original A").unwrap();
        fs::write(root.join("sub/c.md"), "original C").unwrap();

        let files = collect_md_files(&root, true);
        let backup = create_backup(&root, &files, "20260101-120000").unwrap();
        assert!(backup.join("manifest.json").exists());
        assert!(backup.join("a.md").exists());
        assert!(backup.join("sub/c.md").exists());

        // 模拟替换后改动文件
        fs::write(root.join("a.md"), "CHANGED A").unwrap();
        fs::write(root.join("sub/c.md"), "CHANGED C").unwrap();

        let latest = find_latest_backup(&root).unwrap();
        assert_eq!(latest, backup);
        let (_, n) = restore_from_backup(&latest).unwrap();
        assert_eq!(n, 2);
        assert_eq!(fs::read_to_string(root.join("a.md")).unwrap(), "original A");
        assert_eq!(fs::read_to_string(root.join("sub/c.md")).unwrap(), "original C");
    }
}
