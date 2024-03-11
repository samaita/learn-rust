# learn-rust
Dedicated to keeping track on my learning in Rust lang

# Setup

This setup is to build directly with Cargo instead of using just Rust.

## Instalation
For Unix:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


Update
```
rustup update
```

Checking Installation for Cargo(build system and package manager) & Rust
```
learn-rust % cargo --version
cargo 1.76.0 (c84b36747 2024-01-18)
```

Ref: [Rust Getting Started](https://www.rust-lang.org/learn/get-started)

## Setup in VS Code

Find `rust-analyzer` & install

Checking installation for rust
```
learn-rust % rustc --version
rustc 1.76.0 (07dca489a 2024-02-04)
```

Start new project with Cargo
```
cargo new hello_world
cd hello_world
```

the project will looks like
```
src\
    main.rs
.gitignore
Cargo.toml
```

Build the project
```
cargo build
```
You will now have target\debug folder with build output, in MacOS I can try with
```
hello_world % ./target/debug/hello_world
Hello, world!
```

or run the project
```
cargo run
```
Ref: [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)

## Auto Format on Save

make sure to add formater
```
cargo install rustfmt
```
and run on each save
```
cargo fmt
```
then add to `rust-analyzer` settings.json
```
"[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
}
```


Ref#2: [How to run cargo fmt on save in vscode?](https://stackoverflow.com/questions/67859926/how-to-run-cargo-fmt-on-save-in-vscode)

# Learning Source

1. [The Book : The Rust Programming Language](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

# State of Development Around Rust
1. [Are We Game Yet?](https://arewegameyet.rs/)
2. [Are We Web Yet?](https://www.arewewebyet.org/)
3. [Are We Learning Yet?](https://www.arewelearningyet.com/)