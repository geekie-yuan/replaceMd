// 全局响应式状态 + 动作。组件直接 import 这个单例，避免逐层透传 props。
// 这是 Svelte 5 的 runes 模式：在 .svelte.ts 模块里用 $state 定义可跨组件共享的状态。

import { confirm } from '@tauri-apps/plugin-dialog';
import {
  apply,
  pickDirectory,
  preview,
  restoreLatestBackup,
  type PreviewResult,
  type RuleArgs
} from '$lib/api';

export const app = $state({
  folder: '',
  find: '',
  replace: '',
  isRegex: false,
  caseSensitive: false,
  recursive: true,
  preview: null as PreviewResult | null,
  selected: {} as Record<string, boolean>,
  busy: false,
  status: '',
  error: ''
});

function ruleArgs(): RuleArgs {
  return {
    find: app.find,
    replace: app.replace,
    isRegex: app.isRegex,
    caseSensitive: app.caseSensitive
  };
}

/** 当前勾选、且仍在预览结果中的文件路径。 */
export function selectedPaths(): string[] {
  if (!app.preview) return [];
  return app.preview.files.filter((f) => app.selected[f.path]).map((f) => f.path);
}

export async function chooseFolder(): Promise<void> {
  const dir = await pickDirectory();
  if (dir) {
    app.folder = dir;
    app.preview = null;
    app.selected = {};
    app.status = '';
    app.error = '';
  }
}

export async function runPreview(): Promise<void> {
  if (!app.folder) {
    app.error = '请先选择一个目录';
    return;
  }
  if (!app.find) {
    app.error = '请填写查找内容';
    return;
  }
  app.busy = true;
  app.error = '';
  app.status = '';
  try {
    const res = await preview(app.folder, app.recursive, ruleArgs());
    app.preview = res;
    const sel: Record<string, boolean> = {};
    for (const f of res.files) sel[f.path] = true; // 默认全选
    app.selected = sel;
    app.status = `扫描 ${res.scanned_files} 个 .md，命中 ${res.matched_files} 个文件、共 ${res.total_matches} 处`;
  } catch (e) {
    app.preview = null;
    app.error = String(e);
  } finally {
    app.busy = false;
  }
}

export async function runApply(): Promise<void> {
  const paths = selectedPaths();
  if (paths.length === 0) {
    app.error = '请至少勾选一个文件';
    return;
  }
  const ok = await confirm(`将修改 ${paths.length} 个文件，执行前会自动备份。确定继续？`, {
    title: '确认替换',
    kind: 'warning'
  });
  if (!ok) return;

  app.busy = true;
  app.error = '';
  app.status = '';
  try {
    const res = await apply(app.folder, paths, ruleArgs());
    if (res.changed_files === 0) {
      app.status = '没有文件被实际修改';
    } else {
      app.status = `已替换 ${res.changed_files} 个文件、共 ${res.total_replacements} 处。备份目录：${res.backup_dir}`;
    }
    if (res.errors.length) app.error = res.errors.join('\n');
  } catch (e) {
    app.error = String(e);
  } finally {
    app.busy = false;
  }
  // 替换完成后刷新预览（命中应归零，作为完成确认）
  await runPreview();
}

export async function runRestore(): Promise<void> {
  if (!app.folder) {
    app.error = '请先选择目录';
    return;
  }
  const ok = await confirm('将从最近一次备份还原该目录下的文件，覆盖当前内容。确定？', {
    title: '确认还原',
    kind: 'warning'
  });
  if (!ok) return;

  app.busy = true;
  app.error = '';
  app.status = '';
  try {
    const res = await restoreLatestBackup(app.folder);
    app.status = `已从备份还原 ${res.restored_count} 个文件（${res.restored_from}）`;
  } catch (e) {
    app.error = String(e);
  } finally {
    app.busy = false;
  }
  if (app.find) await runPreview();
}

export function toggleFile(path: string): void {
  app.selected = { ...app.selected, [path]: !app.selected[path] };
}

export function setAll(value: boolean): void {
  if (!app.preview) return;
  const sel: Record<string, boolean> = {};
  for (const f of app.preview.files) sel[f.path] = value;
  app.selected = sel;
}
