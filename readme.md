# CCC
## Crusty C Compiler
Compiler for a subset of C written in Rust. Mostly made to learn more Rust

Tokenizer, recursive descent parer and assembly generator for a subset of c. No anlyzer yet

Following [This blog series](https://norasandler.com/2017/11/29/Write-a-Compiler.html) by [Nora Sandler](https://github.com/nlsandler)

### Limitations

Only varianle type is int, which is actually generated as a 64 bit long

no pointers or arrays

### Usage
Requires gcc (Compiles the generated assembly file)

`ccc ./in.c`

compiles ./in.c to ./in same directory and name
