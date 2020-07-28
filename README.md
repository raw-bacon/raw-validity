
![Rust](https://github.com/raw-bacon/raw-validity/workflows/Rust/badge.svg)


# raw-validity
This is a program written in Rust, checking validity of l-group equations and inequations using an [algorithm](https://arxiv.org/abs/1809.02574) developed by George Metcalfe and Almudena Colacito (After Proposition 2 in Version 1).

# Usage
Upon running raw-validity you are prompted to enter either an equation or an inequation. The short version of what you need to know is this.
1. Names of variables are characters, possibly followed by a number, e.g., `x` or `x31`.
2. Inverses of variables are denoted by their capital versions. E.g., the inverse of `x` is `X`. 
3. Meets and joins are denoted by `^` and `v`, respectively. This means that the letter `v` can not appear in the name of a variable.
4. Inverses are denoted by **prefix** `-`. E.g., the inverse of `x v y` is `-(x v y)`.
5. Whitespace and non-alphanumeric characters except `^`, `-`, `(`, and `)` are ignored. In particular, products do not have a symbol. The product of `x v y` and `z ^ w` is `(x v z)(z ^ w)`.

# Installation
In the releases section of this repository, download the file named `exec` and save it somewhere. Open a terminal in the directory containing `exec` and run `chmod +x exec` and then `./exec` (notice the dot). If you want to start the program a second time, it suffices to run `./exec`.

# Alternative
In case the above has not worked for some reason, install Rust and run `cargo run` in a clone of this repository. Note, however, that this will be a much slower experience since this way you don't get the glorious performance of the release version constructed with `cargo build --release`. Consider running the release version in a Docker container instead.
