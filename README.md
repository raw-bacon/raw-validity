
![Rust](https://github.com/raw-bacon/raw-validity/workflows/Rust/badge.svg)


# raw-validity
This is a program checking validity of l-group equations and inequations using an algorithm developed by [George Metcalfe and Amudena Colacito](https://arxiv.org/abs/1809.02574) (it's described after Proposition 2).

# Usage
Try it out with the [webapp](https://raw-bacon.github.io/raw-validity-webapp).
Upon running raw-validity you are prompted to enter either an equation or an inequation. The short version of what you need to know is this.
1. Names of variables are characters, possibly followed by a number, e.g., `x` or `x31`.
2. Inverses of variables are denoted by their capital versions. E.g., the inverse of `x` is `X`. 
3. Meets and joins are denoted by `^` and `v`, respectively. This means that the letter `v` can not appear in the name of a variable.
4. Inverses are denoted by **prefix** `-`. E.g., the inverse of `x v y` is `-(x v y)`.
5. Whitespace and non-alphanumeric characters except `^`, `-`, `(`, and `)` are ignored. In particular, products do not have a symbol. The product of `x v y` and `z ^ w` is `(x v z)(z ^ w)`.
6. Equations are two l-group terms, separated by the symbol `=`. Similarly, separating terms with `<=` encodes an inequation.

If you don't know what else to enter, try `xyz ^ rst <= xsz v ryt`. Try the same in [pyvalidity](https://github.com/raw-bacon/pyvalidity) to see how much the performance has improved!

# Installation
Install Rust and run `cargo run` in a clone of this repository. For more speed, do
```
cargo build --release
cd target/release
chmod +x exec
./exec
```
