# CCC
## Crusty C Compiler
Toy project making a c compiler in rust 

Just a tokenizer, recursive descent parer and assembly generator for a subset of c.

Following [This blog series](https://norasandler.com/2017/11/29/Write-a-Compiler.html) by [Nora Sandler](https://github.com/nlsandler)

### Limitations

Only varianle type is int, which is actually generated as a 64 bit long

no pointers or arrays

### Usage
Requires gcc (Compiles the generated assembly file)

`ccc ./in.c`

compiles ./in.c to ./in same directory and name