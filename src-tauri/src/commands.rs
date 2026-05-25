//! Tauri command 层：串联 replace（纯逻辑）与 fs_ops（文件系统）。
//! 前端通过 `invoke("preview" | "apply" | "restore_latest_backup", ...)` 调用。

use crate::fs_ops::{collect_md_files, create_backup, find_latest_backup, restore_from_backup};
use crate::replace::{analyze, apply_rule, build_regex, Snippet};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

const MAX_SNIPPETS_PER_FILE: usize = 20;

#[derive(Serialize)]
pub struct FilePreview {
    pub path: String,
    pub match_count: usize,
    pub snippets: Vec<Snippet>,
}

#[derive(Serialize)]
pub struct PreviewResult {
    pub files: Vec<FilePreview>,
    pub scanned_files: usize,
    pub matched_files: usize,
    pub total_matches: usize,
    pub errors: Vec<String>,
}

#[derive(Serialize)]
pub struct ApplyResult {
    pub backup_dir: String,
    pub changed_files: usize,
    pub total_replacements: usize,
    pub errors: Vec<String>,
}

#[derive(Serialize)]
pub struct RestoreResult {
    pub restored_from: String,
    pub restored_count: usize,
}

fn now_timestamp() -> String {
    chrono::Local::now().format("%Y%m%d-%H%M%S").to_string()
}

/// 扫描目录并预览：返回每个有命中的文件的命中数与片段。
#[tauri::command]
pub fn preview(
    path: String,
    recursive: bool,
    find: String,
    replace: String,
    is_regex: bool,
    case_sensitive: bool,
) -> Result<PreviewResult, String> {
    let re = build_regex(&find, is_regex, case_sensitive)?;
    let root = Path::new(&path);
    if !root.is_dir() {
        return Err("所选路径不是有效目录".to_string());
    }

    let all = collect_md_files(root, recursive);
    let scanned_files = all.len();
    let mut files = Vec::new();
    let mut errors = Vec::new();
    let mut total_matches = 0usize;

    for f in &all {
        match fs::read_to_string(f) {
            Ok(content) => {
                let (count, snippets) =
                    analyze(&content, &re, &replace, is_regex, MAX_SNIPPETS_PER_FILE);
                if count > 0 {
                    total_matches += count;
                    files.push(FilePreview {
                        path: f.to_string_lossy().to_string(),
                        match_count: count,
                        snippets,
                    });
                }
            }
            Err(e) => errors.push(format!("{}：读取失败（{e}）", f.display())),
        }
    }

    Ok(PreviewResult {
        matched_files: files.len(),
        files,
        scanned_files,
        total_matches,
        errors,
    })
}

/// 执行替换：先备份将改动的文件，再写入。备份失败则整体中止。
#[tauri::command]
pub fn apply(
    target_root: String,
    files: Vec<String>,
    find: String,
    replace: String,
    is_regex: bool,
    case_sensitive: bool,
) -> Result<ApplyResult, String> {
    let re = build_regex(&find, is_regex, case_sensitive)?;
    let root = PathBuf::from(&target_root);

    // 第一遍：计算新内容，只保留真正会变化的文件
    let mut planned: Vec<(PathBuf, String, usize)> = Vec::new();
    let mut errors = Vec::new();
    for fp in &files {
        let p = PathBuf::from(fp);
        match fs::read_to_string(&p) {
            Ok(content) => {
                let (new, count) = apply_rule(&content, &re, &replace, is_regex);
                if count > 0 && new != content {
                    planned.push((p, new, count));
                }
            }
            Err(e) => errors.push(format!("{}：读取失败（{e}）", p.display())),
        }
    }

    if planned.is_empty() {
        return Ok(ApplyResult {
            backup_dir: String::new(),
            changed_files: 0,
            total_replacements: 0,
            errors,
        });
    }

    // 备份（失败则中止，绝不在无备份的情况下改文件）
    let to_backup: Vec<PathBuf> = planned.iter().map(|(p, _, _)| p.clone()).collect();
    let backup_dir = create_backup(&root, &to_backup, &now_timestamp())?;

    // 第二遍：写入
    let mut changed_files = 0usize;
    let mut total_replacements = 0usize;
    for (p, new, count) in &planned {
        match fs::write(p, new) {
            Ok(_) => {
                changed_files += 1;
                total_replacements += *count;
            }
            Err(e) => errors.push(format!("{}：写入失败（{e}）", p.display())),
        }
    }

    Ok(ApplyResult {
        backup_dir: backup_dir.to_string_lossy().to_string(),
        changed_files,
        total_replacements,
        errors,
    })
}

/// 从该目录最近一次备份一键还原。
#[tauri::command]
pub fn restore_latest_backup(target_root: String) -> Result<RestoreResult, String> {
    let root = PathBuf::from(&target_root);
    let backup = find_latest_backup(&root)?;
    let (restored_from, restored_count) = restore_from_backup(&backup)?;
    Ok(RestoreResult {
        restored_from,
        restored_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn preview_apply_restore_full_flow() {
        let tmp = tempdir().unwrap();
        let root = tmp.path().join("docs");
        fs::create_dir_all(root.join("sub")).unwrap();
        fs::write(root.join("a.md"), "hello foo world").unwrap();
        fs::write(root.join("sub/b.md"), "foo foo").unwrap();
        fs::write(root.join("c.md"), "nothing here").unwrap();

        // 预览
        let pv = preview(
            root.to_string_lossy().to_string(),
            true,
            "foo".into(),
            "bar".into(),
            false,
            true,
        )
        .unwrap();
        assert_eq!(pv.scanned_files, 3);
        assert_eq!(pv.matched_files, 2);
        assert_eq!(pv.total_matches, 3);

        // 只对 a.md 执行（模拟用户取消勾选 sub/b.md）
        let a = root.join("a.md").to_string_lossy().to_string();
        let res = apply(
            root.to_string_lossy().to_string(),
            vec![a.clone()],
            "foo".into(),
            "bar".into(),
            false,
            true,
        )
        .unwrap();
        assert_eq!(res.changed_files, 1);
        assert_eq!(res.total_replacements, 1);
        assert!(!res.backup_dir.is_empty());

        // a.md 已改，sub/b.md 未动
        assert_eq!(fs::read_to_string(root.join("a.md")).unwrap(), "hello bar world");
        assert_eq!(fs::read_to_string(root.join("sub/b.md")).unwrap(), "foo foo");

        // 还原
        let restore = restore_latest_backup(root.to_string_lossy().to_string()).unwrap();
        assert_eq!(restore.restored_count, 1);
        assert_eq!(fs::read_to_string(root.join("a.md")).unwrap(), "hello foo world");
    }
}
