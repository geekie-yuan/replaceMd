// 前端与 Rust 后端的桥接层：类型定义 + invoke 封装 + 目录选择。
// 注意：返回结构的字段名与 Rust serde 输出一致（snake_case）；
// 而 invoke 的参数键用 camelCase，Tauri 会自动转成 Rust 的 snake_case 参数名。

import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

export interface Snippet {
  line_no: number;
  before: string;
  after: string;
}

export interface FilePreview {
  path: string;
  match_count: number;
  snippets: Snippet[];
}

export interface PreviewResult {
  files: FilePreview[];
  scanned_files: number;
  matched_files: number;
  total_matches: number;
  errors: string[];
}

export interface ApplyResult {
  backup_dir: string;
  changed_files: number;
  total_replacements: number;
  errors: string[];
}

export interface RestoreResult {
  restored_from: string;
  restored_count: number;
}

export interface RuleArgs {
  find: string;
  replace: string;
  isRegex: boolean;
  caseSensitive: boolean;
}

/** 弹出系统文件夹选择框，返回所选目录（取消则返回 null）。 */
export async function pickDirectory(): Promise<string | null> {
  const result = await open({ directory: true, multiple: false });
  return typeof result === 'string' ? result : null;
}

export function preview(path: string, recursive: boolean, rule: RuleArgs): Promise<PreviewResult> {
  return invoke('preview', {
    path,
    recursive,
    find: rule.find,
    replace: rule.replace,
    isRegex: rule.isRegex,
    caseSensitive: rule.caseSensitive
  });
}

export function apply(targetRoot: string, files: string[], rule: RuleArgs): Promise<ApplyResult> {
  return invoke('apply', {
    targetRoot,
    files,
    find: rule.find,
    replace: rule.replace,
    isRegex: rule.isRegex,
    caseSensitive: rule.caseSensitive
  });
}

export function restoreLatestBackup(targetRoot: string): Promise<RestoreResult> {
  return invoke('restore_latest_backup', { targetRoot });
}
