
![Rust](https://github.com/raw-bacon/rsvalidity/workflows/Rust/badge.svg)


# rsvalidity
This is a program checking validity of $l$-group equations and inequations using an [algorithm](https://arxiv.org/abs/1809.02574) developed by George Metcalfe and Almudena Colacito (After Proposition 2 in Version 1).

# Usage
Upon running raw-validity you are prompted to enter either an equation or an inequation. The short version of what you need to know is this.
1. Names of variables are characters, possibly followed by a number, e.g., `x` or `x31`.
2. Inverses of variables are denoted by their capital versions. E.g., the inverse of `x` is `X`. 
3. Meets and joins are denoted by `^` and `v`, respectively. This means that the letter `v` can not appear in the name of a variable.
4. Inverses are denoted by **prefix** `-`. E.g., the inverse of `x v y` is `-(x v y)`.
5. Whitespace and non-alphanumeric characters except `^`, `-`, `(`, and `)` are ignored. In particular, products do not have a symbol. The product of `x v y` and `z ^ w` is `(x v z)(z ^ w)`.

# Installation
