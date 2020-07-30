
![Rust](https://github.com/raw-bacon/raw-validity/workflows/Rust/badge.svg)


# raw-validity
This is a program checking validity of l-group equations and inequations using an algorithm developed by [Almudena Colacito and George Metcalfe](https://arxiv.org/abs/1809.02574) (it's described after Proposition 2).
Try it out with the [web app](https://raw-bacon.github.io/raw-validity-webapp).

# Installation
Install Rust and run `cargo run` in a clone of this repository. For more speed, do
```
cargo build --release
cd target/release
chmod +x exec
./exec
```
The benefit of running it in a terminal is mainly the additional verbosity, which can inform you at which step the algorithm gets stuck, whereas the web app will just seem dead until it has an answer.
