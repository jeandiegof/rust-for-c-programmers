## A (gentle) introduction to Rust

---

## What is Rust?

+++

### according to the official website...

+++

> A language empowering everyone to build reliable and efficient software.

+++

### deeep, but more specificaly, Rust...

+++

is a **compiled** programming language with<br>**no garbage collector**

+++

uses **LLVM** for code generation

+++

combines **low-level performance**<br>with **high-level features** (iterators, std, async)

+++

**may or may not** be an OOP language<br>(it depends on your definition of OOP)

+++

## Why Rust?

+++

Rust can provide you three things.

+++

(and they should be enough for most cases)

+++

**productivity**<br> through a good toolchain

+++

**performance**<br>through memory efficiency and safety

+++

**reliability**<br> through its unique type system and ownership model

+++

## Who is using Rust?

+++

Mozilla, Amazon, Microsoft, Discord, Dropbox, Figma, Google, Facebook, npm<br>...

---

## Installation

+++

https://www.rust-lang.org/tools/install

+++

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

+++

this installs the rust toolchain<br>(**rustup**, **rustc** and **cargo**)

---

## Hello World

+++

### Creating a project

```shell
cargo new hello-world
```

+++

```
> tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

+++

```Rust
fn main() {
    println!("Hello, world!");
}
```

+++

```
> cargo run
   Compiling hello-world v0.1.0 (/home/user/dev/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/hello-world`
Hello, world!
```

+++

```toml
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
```

+++

### Hello world + random number

+++

```toml
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```

+++

```Rust
fn main() {
    println!("Hello, world! {}", rand::random::<u8>());
}
```

+++

```
> cargo run
   Compiling libc v0.2.144
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.9
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling hello-world v0.1.0 (/home/user/dev/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 1.67s
     Running `target/debug/hello-world`
Hello, world! 197
```

+++

Crates are like libraries, and they come from a **registry**.

+++

**crates.io** is the community (and the default) registry.

+++

but alternate registries are supported.

+++

The toolchain let you focus on **what really matters**.<br>

+++

Create the project, add dependencies, add a test framework, manage
multiple build types or targets **should not take of your valuable time**.

+++

This is **productivity**.

---

## Examples

---

## Extras

- build types: release and debug
- unit test framework: cargo test

---

## Conclusion

