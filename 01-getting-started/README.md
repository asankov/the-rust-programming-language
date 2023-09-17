# 01. Getting Started

Rust is a general-purpose, strongly-typed, compiled language.

It can be used for everything from lower-lever programming to CLIs and web servers.

## 01.2 Hello, World!

[`1.2-hello-world/main.rs`](./1.2-hello-world/main.rs) is a Rust "Hello, World" program.

It contains a `main` function which is the first thing that is executed in a Rust program.

Inside the `main` function we call the `println!` macro (not a function!).

To compile the program run `rustc main.rs`.
This outputs a `main` binary, which is a binary file for our platform.
Running it will produce the `Hello, World` output on the console.

## 01.3 Hello, Cargo!

Cargo is Rust's build system and package manager.

We can use Cargo to create new Rust projects:

```shell
cargo new hello_cargo
```

This creates a new folder with the project name (`hello_cargo`) and some files inside:

- [`Cargo.toml`](./hello-cargo/Cargo.toml)

The Cargo configuration file in a TOML format.

The `project` section contains metadata for the project.

The `dependencies` section contains a list of the project dependencies.

- [`src/`](./hello-cargo/src/)

The `src` directory is where Cargo expects up to keep our source files.

The top-level directory is only for READMEs, metadata, configuration files, etc.

### Building via Cargo

To build our project via Cargo, run:

```shell
cargo build
```

This will output the binary into the `target/build` directory.

This command also created the [`Cargo.lock`](./hello-cargo/Cargo.lock) file, which contains the exact version of dependencies used to build the code.

### Running via Cargo

We can do:

```shell
cargo run
```

to build and run our code with one command.

### Cargo check

```shell
cargo check
```

This command checks that our code compiles without producing an executable artifact.

### Releasing via Cargo

Once we are ready to release our code, we need to build it with the `--release` flag:

```shell
cargo build --release
```

This will produce an executable in the `target/release` folder.
This one is different that the debug executable, because it's optimized for production
