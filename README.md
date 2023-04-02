# ktc32-com

[![Rust](https://github.com/kinpoko/ktc32-com/actions/workflows/rust.yml/badge.svg)](https://github.com/kinpoko/ktc32-com/actions/workflows/rust.yml)
![License](https://img.shields.io/github/license/kinpoko/ktc32-com?color=blue)

ktc32-com is a compiler for the [KTC32](https://github.com/kinpoko/ktc32), written in Rust. It can compile programs like C.

## Build

```bash
git clone https://github.com/kinpoko/ktc32-com.git
cargo build --release
```

## Usage

```bash
ktc32-com test.ktc > test.asm
```

## Reference

[低レイヤを知りたい人のための C コンパイラ作成入門](https://www.sigbus.info/compilerbook)
