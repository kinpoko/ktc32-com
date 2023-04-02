# ktc32-com

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
