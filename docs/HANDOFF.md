# tig 项目交接文档

## 项目目标

以学习为目的，用 Rust 从零实现一个简化版 git，命名为 tig。

目标不是替代 git，而是：
- 通过实现 git 核心机制掌握 Rust 语言特性
- 覆盖所有权、错误处理、文件 IO、模块化、迭代器、泛型、Trait 等核心概念
- 保持 git 对象格式兼容（hash 值可与真实 git 交叉验证）

## 教学方式

- 每轮布置一个任务，用户自己实现
- 实现后 AI 读取源文件、运行检查、给出审查意见
- 检查命令：`cargo check && cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings`
- 验证方式：与 `git` 命令交叉对比对象 hash

## 当前项目状态（截至 2026-09-16）

### 已实现命令

| 命令 | 文件 | 状态 |
|------|------|------|
| `tig init` | `src/commands/init.rs` | ✅ 达标 |
| `tig hash-object <file>` | `src/commands/hash_object.rs` | ✅ 达标 |
| `tig cat-file <hash>` | `src/commands/cat_file.rs` | ✅ 达标 |
| `tig write-tree` | `src/commands/write_tree.rs` | ✅ 达标 |
| `tig commit-tree <tree> -m <msg> [-p <parent>]` | `src/commands/commit_tree.rs` | ✅ 达标 |
| `tig config get/set <key>` | `src/commands/config.rs` | ✅ 达标 |
| `tig log` | `src/commands/log.rs` | ⬜ 空壳，未实现 |

### 目录结构

```
src/
├── main.rs                    # 命令行入口，parse_command 分发
├── commands/
│   ├── mod.rs                 # get_hash() 函数
│   ├── tig_command.rs         # TigCommand trait（get_name/exec/rollback）
│   ├── init.rs
│   ├── hash_object.rs
│   ├── cat_file.rs
│   ├── write_tree.rs
│   ├── commit_tree.rs
│   ├── config.rs
│   └── log.rs
├── models/
│   └── path_models.rs         # PathModel/PathModels/Ignore（早期设计，当前仅 ignore_util 沿用部分思路）
└── utils/
    ├── mod.rs
    ├── blob_util.rs           # get_blob(content) -> Vec<u8>
    ├── tree_util.rs           # write_tree(dir, ignore_rules) -> String
    ├── commit_util.rs         # 空文件，占位
    ├── object_util.rs         # zlib_and_save_to_file / is_object_file_exist / is_valid_object_file
    ├── config_util.rs         # 读写 ~/.tigconfig（INI 格式）
    ├── ignore_util.rs         # load_ignore() / matches_ignore()
    └── zlib_util.rs           # encode / decode（zlib 压缩/解压）
```

### 依赖

```toml
anyhow = "1.0"        # 错误处理
sha1_smol = "1.0"     # SHA-1 计算
flate2 = "1.1"        # zlib 压缩
hex = "0.4"           # hex 编码/解码
dirs = "7.0"          # 获取 home 目录
rust-ini = "0.21"     # 读写 ~/.tigconfig
chrono = "0.4"        # 时区格式化（commit 时间戳）
```

### 关键设计决策

**TigCommand trait**：所有命令实现 `get_name / exec / rollback`，`exec` 失败时自动调 `rollback`。

**对象存储**：与 git 格式完全兼容。blob/tree/commit 对象都以 zlib 压缩存入 `.tig/objects/<前2位>/<后38位>`。已通过 Python 脚本和 `git cat-file` 交叉验证。

**用户配置**：用 INI 格式存在 `~/.tigconfig`，通过 `tig config set user.name xxx` 设置。commit 时从这里读 author/email。

**忽略规则**：`.tigignore` 支持精确名称（`target`）和后缀通配（`*.log`）。`.tig` 目录硬编码进默认规则。`load_ignore()` 在 `exec()` 层调用一次，通过参数传递给 `write_tree` 递归。

**参数传递**：`parse_command` 接收 `Skip<Args>`，各命令在 `new()` 里自己消费剩余参数。

## 下一轮任务：`tig commit`

实现高层的 `tig commit -m <message>`，把底层操作串联起来。

### 步骤

1. 调用 `write_tree(current_dir, &ignore_rules)` 获得 tree hash
2. 读取 `.tig/HEAD` → 解析出 ref 路径（如 `refs/heads/main`）
3. 读取 `.tig/refs/heads/main` → 获取 parent commit hash（第一次为 `None`）
4. 构建 commit 对象（复用 `commit_tree.rs` 的逻辑，或直接提取成 `commit_util.rs` 中的函数）
5. 把新 commit hash 写入 `.tig/refs/heads/main`

### 输出格式

```
[main abc1234] init commit
```
hash 取前 7 位。

### 提示代码

```rust
// 读 HEAD
let head = std::fs::read_to_string(".tig/HEAD")?;
let ref_path = head.trim()
    .strip_prefix("ref: ")
    .ok_or_else(|| anyhow::anyhow!("HEAD 格式错误"))?;

// 读 parent（第一次不存在）
let branch_file = format!(".tig/{}", ref_path);
let parent = std::fs::read_to_string(&branch_file).ok()
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty());

// 写新 commit hash
std::fs::write(&branch_file, format!("{}\n", new_hash))?;
```

### 验收标准

- `tig commit -m "message"` 正常执行
- 连续两次 commit，第二次的 commit 对象包含 `parent` 字段
- `.tig/refs/heads/main` 内容更新为最新 commit hash
- `tig cat-file <hash>` 可读出完整内容
- `cargo check / fmt / clippy / test` 全过

## 后续待实现（按优先级）

1. **`tig commit`** — 当前任务
2. **`tig log`** — 从 HEAD 沿 parent 链遍历并打印 commit 信息
3. **`tig status`** — 对比工作区和最后一次 commit 的 tree，显示新增/修改/删除
4. **`tig diff`** — 显示文件级别的差异
5. **`tig branch`** — 创建和列出分支
6. **`tig checkout`** — 切换分支，更新工作区文件

## 已知技术债务

- `models/path_models.rs` 中的 `PathModels` 是早期设计遗留，当前主流程未使用，考虑后续删除或在 `tig add` 时复用
- `utils/commit_util.rs` 是空文件，commit 构建逻辑目前在 `commit_tree.rs` 的 `exec()` 中，后续实现 `tig commit` 时应提取到这里复用
- `object_util.rs` 中 `zlib_and_save_to_file` 和 `fs_util.rs` 中的 `save_to_file` 功能重复，后续统一

## 教学风格说明（给下一个 AI 会话）

- 每轮只布置一个主要任务
- 用户表示完成后，必须先读源文件、再跑命令，不能凭描述评价
- 不给完整实现，给方向提示和关键代码片段
- 用中文回复，代码/命令保留英文
- 回复直接，不写套话，不空泛表扬
- 验收时参考 `AGENTS.md` 中的审查结构
