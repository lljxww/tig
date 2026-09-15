# tig

用 Rust 实现的 git 学习项目。

## 简介

tig 是一个以学习为目的、用 Rust 从零实现的简化版 git。目标不是替代 git，而是通过实现 git 的核心机制来掌握 Rust 语言特性和工程实践。

实现过程中会逐步覆盖：所有权与借用、错误处理、文件 IO、模块化、迭代器、测试等 Rust 核心概念。

## 已实现的命令

| 命令 | 说明 |
|------|------|
| `tig init` | 初始化 `.tig/` 仓库目录结构 |
| `tig hash-object <file>` | 计算文件的 blob 对象 hash，写入对象存储 |
| `tig cat-file <hash>` | 读取并输出对象内容 |
| `tig write-tree` | 将当前目录写成 tree 对象，输出 hash |

## 使用

```bash
cargo build

# 初始化仓库
./target/debug/tig init

# 存储文件
./target/debug/tig hash-object README.md

# 读取对象
./target/debug/tig cat-file <hash>

# 写入目录快照
./target/debug/tig write-tree
```

## 忽略规则

在项目根目录创建 `.tigignore` 文件，每行一条规则：

```
target
*.log
.DS_Store
```

支持精确名称匹配和 `*.ext` 后缀通配。

## 对象存储格式

与 git 兼容，对象以 zlib 压缩存储在 `.tig/objects/<前2位>/<后38位>`。

- blob：`blob <size>\0<content>`
- tree：`tree <size>\0<entries>`
- commit：`commit <size>\0<content>`（待实现）

## 开发

```bash
cargo check
cargo test
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```
