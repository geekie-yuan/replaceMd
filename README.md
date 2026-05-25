<div align="center">

# replaceMd

**Markdown 批量文本替换桌面工具**

选一个目录，输入「查找 → 替换」，预览命中后再执行；执行前自动备份，可一键还原。

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?logo=tauri&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-5-FF3E00?logo=svelte&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)
![Platform](https://img.shields.io/badge/platform-Windows-0078D6?logo=windows&logoColor=white)

</div>

---

## 这是什么

当你有一整个目录的 `.md` 文件（笔记、文档、博客、Wiki……），需要把某段固定文本统一替换掉时，手动逐个文件改既繁琐又容易遗漏出错。

**replaceMd** 把这件事变成三步：**选目录 → 填规则 → 预览后一键替换**。它是一个基于 Tauri 的轻量桌面应用，安装包仅 2–3 MB，使用系统自带 WebView，不打包浏览器内核。

## ✨ 功能特性

- **批量替换**：一次处理整个目录（可含子目录）下的所有 `.md` 文件
- **两种匹配模式**：字面量替换，或开启**正则表达式**（支持 `$1` 捕获组）
- **大小写开关**：可选区分 / 不区分大小写
- **先预览再执行**：逐文件显示命中数量，并展开查看**替换前 / 后**的逐行对比（红 / 绿高亮）
- **逐文件勾选**：预览后可取消某些文件，只替换你想改的
- **自动备份**：执行前把**将被改动**的文件复制到 `<目标名>-backup-<时间戳>/`，附带 `manifest.json`
- **一键还原**：从最近一次备份恢复目录内容，出错可立即回滚
- **编码安全**：按 UTF-8 读写，只改命中片段，保留原有换行符与 BOM；非 UTF-8 / 读取失败的文件自动跳过并提示

## 📦 安装（普通用户）

到 [Releases](https://github.com/geekie-yuan/replaceMd/releases) 下载对应安装包并运行：

| 安装包 | 说明 |
|--------|------|
| `replaceMd_x.y.z_x64-setup.exe` | NSIS 安装程序（体积最小，推荐） |
| `replaceMd_x.y.z_x64_en-US.msi` | MSI 安装程序 |

> Windows 10/11 已内置 WebView2，无需额外安装运行时。

## 🚀 使用步骤

1. **选择目录…** —— 选定 `.md` 文件所在文件夹
2. **填写规则** —— 输入「查找」和「替换为」，按需勾选 **正则表达式 / 区分大小写 / 包含子目录**
3. **扫描 / 预览** —— 查看每个文件的命中数；点「查看」展开逐行的前后对比；取消不想改的文件
4. **执行替换** —— 弹出确认（显示将改动的文件数）→ 自动备份 → 写入；完成后显示「改了几个文件、共几处、备份目录在哪」
5. **从最近备份还原**（可选）—— 一键把目录恢复到最近一次替换前的状态

> 替换后会自动重新扫描一次，命中数应归零，作为「已完成」的确认。

## 🛡️ 工作原理与安全机制

- **替换逻辑是纯函数**：核心匹配 / 替换不依赖文件系统（见 `src-tauri/src/replace.rs`），保证可测试、行为可预期。
  - 字面量模式下查找内容会被转义（`a.b` 只匹配 `a.b`，不会像正则那样匹配 `axb`）；替换文本中的 `$1` 原样写入。
  - 正则模式下使用 Rust [`regex`](https://docs.rs/regex) 引擎，支持捕获组展开。
- **备份优先**：执行替换时，先把所有**将被改动**的文件按相对路径复制到备份目录并写入清单，**备份失败则整体中止**，绝不在无备份的情况下改文件。
- **可还原**：`manifest.json` 记录原始根目录与文件相对路径；还原时按清单把备份文件拷回原位。
- **逐文件容错**：单个文件读 / 写失败会被收集并在界面提示，不影响其余文件继续处理。

## 🧱 技术栈

| 层 | 技术 |
|----|------|
| 桌面壳 / 后端 | [Tauri v2](https://tauri.app) + Rust（`walkdir`、`regex`、`chrono`、`serde`） |
| 前端 | [SvelteKit](https://svelte.dev/docs/kit) + Svelte 5 runes + TypeScript（静态 / SPA 模式） |
| 目录选择 | `tauri-plugin-dialog` |

## 🗂️ 项目结构

```
replaceMd/
├─ src/                          # 前端（SvelteKit）
│  ├─ routes/+page.svelte        # 主界面布局 + 全局主题
│  └─ lib/
│     ├─ api.ts                  # invoke 封装 + 类型定义
│     ├─ appState.svelte.ts      # 全局响应式状态 + 动作（Svelte 5 runes）
│     ├─ FolderPicker.svelte     # 选目录
│     ├─ RuleForm.svelte         # 查找/替换输入 + 三个开关
│     ├─ ResultsList.svelte      # 结果表：勾选 + 命中数 + 前后对比片段
│     └─ ActionBar.svelte        # 执行替换 / 还原 + 状态提示
├─ src-tauri/                    # 后端（Rust / Tauri）
│  └─ src/
│     ├─ replace.rs              # 纯替换逻辑（含单元测试）
│     ├─ fs_ops.rs              # 扫描 / 备份 / 还原（含单元测试）
│     ├─ commands.rs             # Tauri 命令 preview/apply/restore（含集成测试）
│     └─ lib.rs                  # 注册插件与命令
└─ test-fixtures/                # 手动验证用示例 .md（含子目录）
```

## 🛠️ 从源码构建（开发者）

### 前置环境

- [Node.js](https://nodejs.org) ≥ 18 与 [pnpm](https://pnpm.io)
- [Rust](https://www.rust-lang.org/tools/install) 稳定版工具链
- Windows：Microsoft C++ Build Tools + WebView2（Win10/11 已内置）
- 其它平台依赖见 [Tauri 前置要求](https://tauri.app/start/prerequisites/)

### 常用命令

```bash
pnpm install        # 安装前端依赖
pnpm tauri dev      # 开发模式（前端热更新 + Rust 后端）
pnpm check          # 前端类型检查（svelte-check）
pnpm tauri build    # 打包安装程序 → src-tauri/target/release/bundle/
```

### 运行测试

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

覆盖：字面量 / 正则 / 大小写 / 无命中 / 特殊字符 / `$` 捕获组等替换逻辑，以及扫描、备份、还原的完整流程。

### 镜像加速（依赖下载慢时）

若 `crates.io` 下载缓慢 / 频繁超时，可在项目根目录建 `.cargo/config.toml` 指向镜像，例如 [rsproxy](https://rsproxy.cn)：

```toml
[source.crates-io]
replace-with = "rsproxy-sparse"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[net]
retry = 5
```

## 🗺️ 路线图

当前为 v1：单条「查找 → 替换」规则。后续考虑（欢迎 Issue / PR 讨论优先级）：

- [ ] 多条规则批量配置，一次执行
- [ ] 规则导入 / 导出（CSV / JSON），便于复用与版本管理
- [ ] 扩展文件类型（不限于 `.md`）与排除规则（如忽略某些目录）
- [ ] 备份历史浏览与选择性还原
- [ ] 深色主题 / 国际化

## 🤝 贡献

欢迎提交 Issue 和 Pull Request。建议流程：

1. Fork 并新建分支
2. 改动后跑通 `cargo test` 与 `pnpm check`
3. 提交 PR 并简要说明动机与改动点

新功能建议先开 Issue 讨论，避免与路线图或既有设计冲突。

## 📄 许可证

本项目基于 [MIT License](./LICENSE) 开源。
