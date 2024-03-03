# learn-rust
Dedicated to keeping track on my learning in Rust lang

# Setup

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

