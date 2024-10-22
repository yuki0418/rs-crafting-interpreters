# RS CRAFTING INTERPRETERS

This project is a Rust implementation of the Crafting Interpreters book by Bob Nystrom.
[CRAFTING INTERPRETERS] (https://craftinginterpreters.com/)

Some of the code is based on the Java implementation of the book, but I'm trying to make it more Rust idiomatic.

## Scanner

The scanner is the first part of the interpreter. It reads the source code and converts it into tokens.

You can play examples with the scanner by running the following command:

```bash
cargo run --example scanner examples/scanner/test.txt
```