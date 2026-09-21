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

## 当前项目状态（截至 2026-09-21）

### 已实现命令

| 命令 | 文件 | 状态 |
|------|------|------|
| `tig init` | `src/commands/init.rs` | ✅ 达标 |
| `tig hash-object <file>` | `src/commands/hash_object.rs` | ✅ 达标 |
| `tig cat-file <hash>` | `src/commands/cat_file.rs` | ✅ 达标 |
| `tig write-tree` | `src/commands/write_tree.rs` | ✅ 达标 |
| `tig commit-tree <tree> -m <msg> [-p <parent>]` | `src/commands/commit_tree.rs` | ✅ 达标 |
| `tig config get/set <key>` | `src/commands/config.rs` | ✅ 达标 |
| `tig commit -m <message>` | `src/commands/commit.rs` | ✅ 达标 |
| `tig log` | `src/commands/log.rs` | ✅ 达标 |
| `tig status` | `src/commands/status.rs` | ✅ 达标 |
| `tig diff` | `src/commands/diff.rs` | ✅ 达标 |
| `tig branch [name]` | `src/commands/branch.rs` | ✅ 达标 |
| `tig checkout <branch>` | `src/commands/checkout.rs` | ✅ 达标 |

### 目录结构

```
src/
├── main.rs                        # 命令行入口，parse_command 分发
├── commands.rs                    # pub mod 声明
├── commands/
│   ├── tig_command.rs             # TigCommand trait（get_name/exec/rollback）
│   ├── init.rs
│   ├── hash_object.rs
│   ├── cat_file.rs
│   ├── write_tree.rs
│   ├── commit_tree.rs
│   ├── config.rs
│   ├── commit.rs
│   ├── log.rs
│   ├── status.rs
│   ├── diff.rs
│   ├── branch.rs
│   └── checkout.rs
├── models.rs                      # pub mod 声明
├── models/
│   └── objects.rs                 # get_object_path / get_content_from_raw / is_valid_object_file
│   └── objects/
│       ├── tig_object.rs          # TigObject trait（content/hash/raw/store）
│       ├── blob.rs                # Blob：from_file（工作区）/ from_hash（对象库）/ text
│       ├── tree.rs                # Tree：new / from_hash / list_files / restore_to_dir
│       └── commit.rs             # Commit：new / from_hash / tree_hash / parent_hash / get_print_text
├── utils.rs                       # pub mod 声明
└── utils/
    ├── repo_util.rs               # HEAD/ref/分支读写（get_last_commit_hash / get_all_branches / set_head_to_branch）
    ├── working_dir_util.rs        # scan_working_dir（工作区扫描，不写磁盘）
    ├── diff_util.rs               # Hirschberg LCS diff 算法（DiffLine / diff_lines）
    ├── zlib_util.rs               # encode / decode（zlib 压缩/解压）
    ├── ignore_util.rs             # load_ignore() / matches_ignore()
    ├── config_util.rs             # 读写 ~/.tigconfig（INI 格式）
    └── colored_print_util.rs      # 终端颜色输出（检测 is_terminal，重定向时降级）
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

**TigObject trait**：三种对象（Blob/Tree/Commit）都实现 `TigObject`，trait 提供默认方法：
- `hash()` → SHA1(`content()`)
- `raw()` → `"{type} {len}\0{content}"`
- `store()` → 压缩后写入 `.tig/objects/{前2}/{后38}`

**对象路径**：统一通过 `get_object_path(hash)` 获取，不再各处手动拼接。

**对象读取 vs 工作区读取**：
- `Blob::from_hash(hash)` → 从 `.tig/objects/` 读，内部调 `get_content_from_raw`（解压）
- `Blob::from_file(path)` → 从工作区读普通文件，**不解压**
- 两者不能混用，之前曾因混用导致 `corrupt deflate stream` bug

**commit 时序**：`Commit::new()` 调用时确定 author/timestamp，存入结构体字段。`content()` 只做拼接，保证 `store()` 和 `hash()` 的结果一致。

**用户配置**：INI 格式存在 `~/.tigconfig`，`tig config set user.name xxx` 设置。commit 时读取。

**忽略规则**：`.tigignore` 支持精确名称（`target`）和后缀通配（`*.log`）。`.tig` 目录硬编码进默认规则。

**分支机制**：
- HEAD 文件内容：`ref: refs/heads/main\n`
- 分支文件：`.tig/refs/heads/<name>`，内容是最新 commit hash
- `repo_util.rs` 统一提供 HEAD/分支相关读写操作

**diff 算法**：Hirschberg LCS（空间 O(min(N,M))），先剔除公共前后缀再对中间区域做 LCS，输出 `DiffLine::Context / Added / Removed`。

**参数传递**：`parse_command` 接收 `Skip<Args>`，各命令在 `new()` 里自己消费剩余参数。

## 已知技术债务

- `Tree::new` 构建时会对所有子树和 blob 调 `store()`，即使目的只是计算 hash（如 `status` 扫描时不需要写磁盘）。目前 `scan_working_dir` 用 `Blob::from_file + hash()` 绕开了这个问题，但 Tree 对象还是会写磁盘。后续可以给 TigObject 加 `hash_only()` 路径。
- `checkout` 切换分支时直接覆盖工作区文件，不检查未提交的本地修改，可能丢失数据。后续应先做 dirty check。
- 没有 index（暂存区），`tig commit` 直接提交整个工作区，无法做部分提交。
- `tig log` 的 `get_print_text` 只打印 hash 前 7 位和 message，没有 author 和日期。
- 单元测试很少，只有 `zlib_util` 有一个 roundtrip 测试。`diff_util`、`ignore_util`、`repo_util` 等纯函数都缺测试。

## 下一步候选任务（按优先级）

1. **补测试**：为 `diff_util`、`ignore_util`、`Tree::list_files`、`Blob` 等加单元测试和集成测试，是目前最值得做的工程质量提升
2. **`tig log` 完善**：打印 author、日期，支持 `--oneline` 参数
3. **`tig add` + index**：实现暂存区，让 commit 只提交 staged 文件
4. **checkout dirty check**：切换前检查工作区是否有未提交修改
5. **`tig stash`**：暂存工作区变更

## 教学风格说明（给下一个 AI 会话）

- 每轮只布置一个主要任务
- 用户表示完成后，必须先读源文件、再跑命令，不能凭描述评价
- 不给完整实现，给方向提示和关键代码片段
- 用中文回复，代码/命令保留英文
- 回复直接，不写套话，不空泛表扬
- 验收时参考 `AGENTS.md` 中的审查结构
- 用户当前水平：已掌握所有权/借用/生命周期基础、Result/Option 错误处理、Trait 设计、泛型、模块化、文件 IO、迭代器；能独立完成中等复杂度 Rust 任务；下一步可以引入异步、测试框架深度使用、性能分析等话题
