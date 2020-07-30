
![Rust](https://github.com/raw-bacon/raw-validity/workflows/Rust/badge.svg)


# raw-validity
This is a program checking validity of l-group equations and inequations using an algorithm developed by [Almudena Colacito and George Metcalfe](https://arxiv.org/abs/1809.02574) (it's described after Proposition 2).

# Usage
Try it out with the [webapp](https://raw-bacon.github.io/raw-validity-webapp).

If you don't know what else to enter, try `xyz ^ rst <= xsz v ryt`. Try the same in [pyvalidity](https://github.com/raw-bacon/pyvalidity) to see how much the performance has improved!

# Installation
Install Rust and run `cargo run` in a clone of this repository. For more speed, do
```
cargo build --release
cd target/release
chmod +x exec
./exec
```
