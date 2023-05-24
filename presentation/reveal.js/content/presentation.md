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

**reliability**<br> through its unique type system and ownership model

+++

**performance**<br>through memory efficiency and safety

+++

## Who is using Rust?

+++

Mozilla, Amazon, Microsoft, Discord, Dropbox, Figma, Google, Facebook, npm<br>...

+++

yes, it is production ready (for most domains).

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

```shell
> tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

+++

src/main.rs

```Rust
fn main() {
    println!("Hello, world!");
}
```

+++

```shell
> cargo run
   Compiling hello-world v0.1.0 (/home/user/dev/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/hello-world`
Hello, world!
```

+++

Cargo.toml

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

Cargo.toml

```toml [0-0|6-7]
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```

+++

src/main.rs

```rust
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

The `Cargo.toml` is _like_ a `CMakelists.txt` or a `meson.build`

+++

Crates are _like_ libraries, and they come from a **registry**.

+++

**crates.io** is the community (and also the default) registry.

+++

but alternate registries are supported.

+++

The toolchain let you focus on **what really matters**.<br>

+++

Create the project, add dependencies, add a test framework, manage
multiple build types or targets **should not take of your valuable time**.

+++

This is **productivity**.<br>

---

## Show me the code

+++

### Variables

```rust [1-2|4-5|7-9|11-12|14-15|17-18|20-22|24-25]
// compiler can infer the type
let x = 10;

// or we can specify it
let y: u8 = 255;

// we also have floats
let y: f32 = 124.98;
let y: f64 = 124.98;

// string literals
let s = "hi";

// characters
let c = 'z';

// booleans
let b = true;

// n-tuples
let point_2d = (10, 0);
let point_3d = (10, 0, 4);

// arrays
let some_numbers = [1, 2, 3, 4, 5];
```

+++

### Mutability

+++

Is this code ok?

```rust [1-4|2|3]
fn main() {
  let x = 10;
  x = x + 5;
}
```

+++

not in Rust.

```shell
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 10;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     x = x + 5;
  |     ^^^^^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
```

+++

a possible alternative

```rust [1|1-2]
let mut x = 10;
x = x + 5;
```

+++

a possible alternative

```rust [1-2|2]
let x = 10;
let x = x + 5;
```

+++

variables are mutable only **when you explicitly say so**

+++

what is wrong with mutable variables by default?

+++

Let's take a look at a C example

```C [1-7|2|4|4-6]
int main(void) {
  int x = read_int_from_stdin();

  if (x = 10) {
    printf("x is 10.");
  }
}

```

+++

the correct version

```C [1-7|2|4|1-7]
int main(void) {
  int const x = read_int_from_stdin();

  if (x == 10) {
    printf("x is 10.");
  }
}
```

+++

### Ownership

+++

**Data** in Rust can only have **one owner at a time**

+++

and it **exists** only while its **owner exist**.

+++

what should happen in the scenario below?

```rust
fn main() {
  let s1 = String::from("Hello");
  let s2 = s1;

  println!("{:?}", s1);
}
```

+++

in Rust, it does not compile.

```shell [1-11|1|9-10|6-7]
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:22
  |
2 |     let s1 = String::from("Hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{:?}", s1);
  |                      ^^ value borrowed here after move
  |
```

+++

but this is valid in C

```C [1-13|1-6|8-13]
char * allocate_string(char * str) {
  char * dest = malloc(strlen(str) * sizeof(char));
  strncpy(dest, str, strlen(str));

  return dest;
}

int main(void) {
  char * s1 = allocate_string("hello");
  char * s2 = s1;

  printf("%s\n", s1);
}
```

+++

what if we free s1?

```C [1-15|12|14]
char * allocate_string(char * str) {
  char * dest = malloc(strlen(str) * sizeof(char));
  strncpy(dest, str, strlen(str));

  return dest;
}

int main(void) {
  char * s1 = allocate_string("hello");
  char * s2 = s1;

  free(s1);

  printf("%s\n", s2);
}
```

+++

s2 is a **dangling pointer**,<br>it does not point to valid data anymore.

+++

The **ownership model** catch this kind of issues **in compile time**.

+++

So, we have seen that the following is not valid in Rust

```Rust
fn main() {
  let s1 = String::from("Hello");
  let s2 = s1;

  println!("{}", s1);
}
```

+++

but does this code compile?

```Rust
fn main() {
  let x = 10;
  let y = x;

  println!("{}", x);
}
```

+++

yes, it does. But why?

```shell
> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello-world`
10
```

+++

**deep copy** versus **shallow copy**

+++

deep copies are **not** done by default

+++

and since data can only have **one owner** at a time,<br>
the string is **moved** to s2

+++

but for the integer, there is no difference between a
shallow and a deep copy, so it is **copied**.

+++

Still about ownership, does the following compiles?

```rust [1-10|1-3|5-10|6|8|9]
fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let hello = String::from("hello");

    print_string(hello);
    println!("{}", hello);
}
```

+++

no, it does not.

```shell
error[E0382]: borrow of moved value: `hello`
 --> src/main.rs:9:20
  |
6 |     let hello = String::from("hello");
  |         ----- move occurs because `hello` has type `String`, which does not implement the `Copy` trait
7 |
8 |     print_string(hello);
  |                  ----- value moved here
9 |     println!("{}", hello);
  |                    ^^^^^ value borrowed here after move
  |
```

+++

how to fix it?

+++

receiving a **reference** to the string instead

```rust [1-10|1|8]
fn print_string(s: &String) {
    println!("{}", s);
}

fn main() {
    let hello = String::from("hello");

    print_string(&hello);
    println!("{}", hello);
}
```

+++

```shell
> cargo run
   Compiling hello-world v0.1.0 (/home/user/dev/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/hello-world`
hello
hello
```

+++

what if we need to **modify** the data inside the function?

+++

does the following compile?

```rust [1-10|1-3|5-10|6|7|9]
fn append_word(s: &String, word: &str) {
    s.push_str(word);
}

fn main() {
    let hello = String::from("hello");
    append_word(&hello, " world!");

    println!("{}", hello);
}
```

+++

no, it does not.

```
error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
 --> src/main.rs:2:5
  |
1 | fn append_word(s: &String, word: &str) {
  |                   ------- help: consider changing this to be a mutable reference: `&mut String`
2 |     s.push_str(word);
  |     ^^^^^^^^^^^^^^^^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

+++

does it compile now?

```rust [1|1-10]
fn append_word(s: &mut String, word: &str) {
    s.push_str(word);
}

fn main() {
    let hello = String::from("hello");
    append_word(&hello, " world!");

    println!("{}", hello);

```

+++

no, it still does not.

```shell [1-10|7|4-5|9-10]
error[E0308]: mismatched types
 --> src/main.rs:7:17
  |
7 |     append_word(&hello, " world!");
  |     ----------- ^^^^^^ types differ in mutability
  |     |
  |     arguments to this function are incorrect
  |
  = expected mutable reference &mut String
               found reference &String
```

+++

and now?

```rust [7|1-10]
fn append_word(s: &mut String, word: &str) {
    s.push_str(word);
}

fn main() {
    let hello = String::from("hello");
    append_word(&mut hello, " world!");

    println!("{}", hello);
}
```

+++

not yet :(

```shell [1-7|6-7|4-5]
error[E0596]: cannot borrow `hello` as mutable, as it is not declared as mutable
 --> src/main.rs:7:17
  |
6 |     let hello = String::from("hello");
  |         ----- help: consider changing this to be mutable: `mut hello`
7 |     append_word(&mut hello, " world!");
  |                 ^^^^^^^^^^ cannot borrow as mutable
```

+++

this is the moment you start cursing the compiler

+++

but remember, it is only protecting you :)

+++

does it compile now?

```rust [6|1-10]
fn append_word(s: &mut String, word: &str) {
    s.push_str(word);
}

fn main() {
    let mut hello = String::from("hello");
    append_word(&mut hello, " world!");

    println!("{}", hello);
}
```

+++

yes 🎉

```shell
> cargo run
   Compiling hello-world v0.1.0 (/home/user/dev/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/hello-world`
hello world!
```

+++

You have to be very permissive to allow your data to be modified.

+++

but what if we were using C?

+++

What does the following program prints?

```C
void print_str(char const * str);

int main(void) {
  char const * str = allocate_string("hello world!");

  print_str(str);
  printf("%s\n", str);

  return 0;
}
```

+++

expected output

```shell
hello world!
hello world!
```

+++

consider the following implementation of `print_str`

```C [1-7|4-6]
void print_str(char const * str) {
  printf("%s\n", str);

  // cast the const away
  char * mut_str = (char *)str;
  strcpy(mut_str, "bye world!");
}

int main(void) {
  char const * str = allocate_string("hello world!");

  print_str(str);
  printf("%s\n", str);

  return 0;
}
```

+++

the code compiles, executes and outputs:

```shell
> ./constexample
hello world!
bye world!
```

+++

multiple refs?
data only lives while its owner is alive
why rust doesn't have a garbage collector?

+++

The **type system** and the **ownership model** together can catch several bugs in compile time.

+++

If your code compiled, you can trust it.

+++

This is **reliability**.

+++

---


## Extras

- rust book
- rust embedded book
- toolchain
  - build types: release and debug
  - unit test framework: cargo test
- lifetimes
- enum, option, result
- generics and traits
- parallel programming

---

## Conclusion

