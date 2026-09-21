# tig

用 Rust 实现的 git 学习项目。

## 简介

tig 是一个以学习为目的、用 Rust 从零实现的简化版 git。目标不是替代 git，而是通过实现 git 的核心机制来掌握 Rust 语言特性和工程实践。

对象存储格式与 git 完全兼容，blob/tree/commit 的 hash 值可以通过 `git cat-file` 交叉验证。

## 已实现的命令

| 命令 | 说明 |
|------|------|
| `tig init` | 初始化 `.tig/` 仓库目录结构 |
| `tig hash-object <file>` | 计算文件的 blob 对象 hash，写入对象存储 |
| `tig cat-file <hash>` | 读取并输出对象内容 |
| `tig write-tree` | 将当前目录写成 tree 对象，输出 hash |
| `tig commit-tree <tree> -m <msg> [-p <parent>]` | 底层 commit 创建命令 |
| `tig commit -m <message>` | 提交当前工作区，自动关联 HEAD 分支 |
| `tig log` | 从 HEAD 沿 parent 链遍历并打印 commit 历史 |
| `tig status` | 对比工作区与上次 commit，显示新增/修改/删除 |
| `tig diff` | 显示工作区与上次 commit 的行级差异（带颜色） |
| `tig branch` | 列出所有分支，当前分支标 `*` |
| `tig branch <name>` | 在当前 commit 上创建新分支 |
| `tig checkout <branch>` | 切换分支，还原工作区文件 |
| `tig config set/get <key>` | 读写 `~/.tigconfig`（用户名、邮箱等） |

## 快速开始

```bash
cargo build

# 初始化仓库
./target/debug/tig init

# 配置用户信息
./target/debug/tig config set user.name "Your Name"
./target/debug/tig config set user.email "you@example.com"

# 提交
./target/debug/tig commit -m "init commit"

# 查看历史
./target/debug/tig log

# 查看状态
./target/debug/tig status

# 查看差异
./target/debug/tig diff

# 分支操作
./target/debug/tig branch dev
./target/debug/tig checkout dev
./target/debug/tig branch
```

## 忽略规则

在项目根目录创建 `.tigignore` 文件，每行一条规则：

```
target
*.log
.DS_Store
```

支持精确名称匹配和 `*.ext` 后缀通配。`.tig` 目录默认忽略。

## 对象存储格式

与 git 兼容，对象以 zlib 压缩存储在 `.tig/objects/<前2位>/<后38位>`。

- blob：`blob <size>\0<content>`
- tree：`tree <size>\0<entries>`（二进制 entry 格式，与 git 一致）
- commit：`commit <size>\0<headers>\n\n<message>`

## 项目结构

```
src/
├── main.rs                    # 命令行入口
├── commands/                  # 各命令实现
│   ├── tig_command.rs         # TigCommand trait
│   ├── init / commit / log / status / diff / branch / checkout ...
├── models/
│   └── objects/               # 对象模型
│       ├── tig_object.rs      # TigObject trait（content/hash/raw/store）
│       ├── blob.rs
│       ├── tree.rs
│       └── commit.rs
└── utils/
    ├── repo_util.rs           # HEAD/ref/分支状态读取
    ├── working_dir_util.rs    # 工作区扫描
    ├── diff_util.rs           # Hirschberg LCS diff 算法
    ├── zlib_util.rs           # zlib 压缩/解压
    ├── ignore_util.rs         # .tigignore 规则
    ├── config_util.rs         # ~/.tigconfig 读写
    └── colored_print_util.rs  # 终端颜色输出
```

## 开发

```bash
cargo check
cargo test
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```
