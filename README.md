# RZIP

RZIP 是一款基于 Rust 的高效压缩工具，目前还在开发中...
## project

rust project，前往 project 目录下执行 cargo run 即可将 src 目录下的 test.txt 压缩成 result.zip

## rlib

rlib 文件是 rust 库文件，用来进行 wasm 打包， 交给 JS 调用。

## Problem

在执行 wasm-pack build --target web 的时候遇到了一个目前解决不了的问题 `failed to run custom build command for `bzip2-sys v0.1.11+1.0.8``