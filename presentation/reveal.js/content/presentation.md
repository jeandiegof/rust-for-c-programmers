## Rust for C programmers

---

## Summary

1. **What** is Rust?
2. **Why** Rust?
3. **Who** is using Rust?
4. Installation
5. Hello World
6. How is Rust **different** from C?
7. Is Rust **_fast_**? Is it **_memory efficient_**?
8. Going further

---

## What is Rust?

+++

### According to the official website...

+++

> A language empowering everyone to build reliable and efficient software.

+++

Poetic, but this alone probably won't convince you.

+++

So, more specificaly, **Rust**...

+++

is a **compiled** programming language with<br>**no garbage collector**

+++

uses **LLVM** for code generation

+++

combines **low-level performance**<br>with **high-level features** (iterators, std, async)

---

## Why Rust?

+++

Rust can provide you three things.

+++

**Productivity**<br> through a good *toolchain*.

+++

**Reliability**<br> through its unique *type system* and *ownership model*.

+++

**Performance**<br>through *memory efficiency* and *safety*.

---

## Who is using Rust?

+++

Mozilla, Amazon, Microsoft, Discord, Dropbox, Figma, Google, Facebook, npm<br>...

+++

Yes, it is **production ready**.

---

## Installation

+++

https://www.rust-lang.org/tools/install

+++

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

+++

This installs the rust toolchain<br>(**rustup**, **rustc** and **cargo**)

---

## Hello World

+++

### Creating a project

```shell
cargo new hello-world
```

+++

### Project structure

```shell
> tree hello-world
hello-world
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

+++

### src/main.rs

```Rust
fn main() {
    println!("Hello, world!");
}
```

+++

### Running the project

```shell
> cargo run
   Compiling hello-world v0.1.0 (/home/user/dev/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/hello-world`
Hello, world!
```

+++

### Cargo.toml

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

### Cargo.toml

```toml [0-0|6-7]
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```

+++

### src/main.rs

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

The *Cargo.toml* is **_like_** a *CMakelists.txt* or a *meson.build*.

+++

Crates are _like_ libraries, and they come from a **registry**.

+++

**crates.io** is the community (and also the default) registry,<br>
but alternate registries are also supported.

+++

The toolchain let you focus on **what really matters**.<br>

+++

Set up the project, add dependencies, add a test framework, manage
multiple build types and targets **should not take of your valuable time**.

+++

This is **productivity**.<br>

---

## How is Rust different from C?

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

Not in Rust.

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

A possible alternative

```rust [1|1-2]
let mut x = 10;
x = x + 5;
```

+++

Another alternative

```rust [1-2|2]
let x = 10;
let x = x + 5;
```

+++

Variables are **mutable** only when you **explicitly** say so.

+++

What is **wrong** with mutable variables **by default**?

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

The correct version

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

What should happen in the scenario below?

```rust
fn main() {
  let s1 = String::from("Hello");
  let s2 = s1;

  println!("{:?}", s1);
}
```

+++

In Rust, it does not compile.

```shell [1-12|1|10-11|7-8]
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:22
  |
2 |     let s1 = String::from("Hello");
  |         -- move occurs because `s1` has type `String`,
  |            which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{:?}", s1);
  |                      ^^ value borrowed here after move
  |
```

+++

But this is valid in C

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

What if we free s1?

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

The **ownership model** catch this kind of issues<br>**in compile time**.

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

But what about this?

```Rust
fn main() {
  let x = 10;
  let y = x;

  println!("{}", x);
}
```

+++

It compiles and runs. But why?

```shell
> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello-world`
10
```

+++

**Deep copy** versus **Shallow copy**

+++

Deep copies are **not** done by default.

+++

And since data can only have **one owner** at a time,<br>
the string is **moved** to s2.

+++

For the integer, however, there is no difference between a
shallow and a deep copy, so it is **copied**.

+++

Still about ownership, does the following compile?

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

No, it does not.

```shell [1-12|1|8-9|10-11]
error[E0382]: borrow of moved value: `hello`
 --> src/main.rs:9:20
  |
6 |     let hello = String::from("hello");
  |         ----- move occurs because `hello` has type `String`,
  |               which does not implement the `Copy` trait
7 |
8 |     print_string(hello);
  |                  ----- value moved here
9 |     println!("{}", hello);
  |                    ^^^^^ value borrowed here after move
  |
```

+++

How to fix it?

+++

### References

+++

We fix it by receiving a **reference** to the string instead.

+++

Receiving a reference as argument

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

What if we need to **modify** the data inside the function?

+++

Does the following compile?

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

No, it does not.

```shell [1-11|1-2|5-8|9-12]
error[E0596]: cannot borrow `*s` as mutable, as it is behind
              a `&` reference
 --> src/main.rs:2:5
  |
1 | fn append_word(s: &String, word: &str) {
  |                   ------- help: consider changing this to be
  |                                 a mutable reference:
  |                                 `&mut String`
2 |     s.push_str(word);
  |     ^^^^^^^^^^^^^^^^ `s` is a `&` reference, so the data
  |                      it refers to cannot be borrowed as
  |                      mutable.
```

+++

Does it compile now?

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

No, it still does not.

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

And now?

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

Not yet :(

```shell [1-9|1-2|8-9|5-7]
error[E0596]: cannot borrow `hello` as mutable, as it is not
              declared as mutable
 --> src/main.rs:7:17
  |
6 |     let hello = String::from("hello");
  |         ----- help: consider changing this to be
  |               mutable: `mut hello`
7 |     append_word(&mut hello, " world!");
  |                 ^^^^^^^^^^ cannot borrow as mutable
```

+++

This is when you start **cursing the compiler**.

+++

But remember, it is only **protecting you**!

+++

Does it compile now?

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

Yes 🎉

```shell
> cargo run
   Compiling hello-world v0.1.0 (/home/user/dev/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/hello-world`
hello world!
```

+++

So, references are **immutable by default**.

+++

If you need it to be **mutable**, you have to use a **mutable reference** instead.

+++

In Rust, you have to be very **explicit** to allow your **data** to be **modified**.

+++

But what if we were using C?

+++

What does line 7 print?

```C [1-10]
void print_str(char const * str);

int main(void) {
  char const * str = allocate_string("hello world!");

  print_str(str);
  printf("%s\n", str);

  return 0;
}
```

+++

Expected output

```shell
hello world!
hello world!
```

+++

Consider the following implementation of `print_str`

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

The code compiles, executes and outputs:

```shell
> ./constexample
hello world!
bye world!
```

+++

In C, even if your libraries are __const correct__, your data still can be modified,
**intentionally or not**.

+++

### Multiple references

+++

**How many references** to a variable are **allowed**?

+++

In C?

+++

It is limited only by memory.

+++

In Rust?

+++

You can have *either* one **mutable** reference or *any* number of **immutable** references.

+++

Two immutable references

```Rust [1-9|4-5|1-9]
fn main() {
    let mut s = String::from("hello");

    let s_ref = &s;
    let s_other_ref = &s;

    println!("{}", s_ref);
    println!("{}", s_other_ref);
}
```

```
> cargo run
hello
hello
```

+++

A single mutable reference

```Rust [1-8|4|1-8]
fn main() {
    let mut s = String::from("hello");

    let s_ref = &mut s;
    s_ref.push_str(" world");

    println!("{}", s_ref);
}
```

```
> cargo run
hello world
```

+++

Two mutable references?

```Rust [1-9|4-5|1-9]
fn main() {
    let mut s = String::from("hello");

    let s_ref = &mut s;
    let s_other_ref = &mut s;

    println!("{}", s_other_ref);
    println!("{}", s_ref);
}
```

+++

```shell [1-13|1-2]
error[E0499]: cannot borrow `s` as mutable more than once
              at a time
 --> src/main.rs:5:23
  |
4 |     let s_ref = &mut s;
  |                 ------ first mutable borrow
  |                        occurs here
5 |     let s_other_ref = &mut s;
  |                       ^^^^^^ second mutable borrow
  |                              occurs here
...
8 |     println!("{}", s_ref);
  |                    ----- first borrow later used here
```

+++

One mutable and one immutable reference?

```Rust [1-9|4-5|1-9]
fn main() {
    let mut s = String::from("hello");

    let s_ref = &mut s;
    let s_other_ref = &s;

    println!("{}", s_other_ref);
    println!("{}", s_ref);
}
```

+++

```shell [1-11|1-2]
error[E0502]: cannot borrow `s` as immutable because it is
              also borrowed as mutable
 --> src/main.rs:5:23
  |
4 |     let s_ref = &mut s;
  |                 ------ mutable borrow occurs here
5 |     let s_other_ref = &s;
  |                       ^^ immutable borrow occurs here
...
8 |     println!("{}", s_ref);
  |                    ----- mutable borrow later used here 
```

+++

But why?

+++

People holding an immutable reference<br>**don't expect the data to change**.

+++

And people holding a mutable reference<br>**don't expect other people to modify the data**.

+++

What kind of **problem** does this rule **solve**?

+++

Data races.

+++

### Dangling references

+++

They don't exist.

+++

🤯

+++

Dangling reference?

```Rust [1-10|1-5|1|2|4|1-10]
fn hello() -> &String {
    let s = String::from("hello");

    &s
}

fn main() {
    let s = hello();
    println!("{}", s);
}
```

+++

```shell
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:15
  |
1 | fn hello() -> &String {
  |               ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value,
    but there is no value for it to be borrowed from
```

+++

References must **always be valid** and<br>this is **enforced by the compiler**.

+++

Being valid means that the **data won't go out of scope before the reference does**.

+++

There is **no need** for a **garbage collector**.

+++

The **type system** and the **ownership model** together can catch several bugs in compile time.

+++

If your **code compiled**, you can **trust it**.

+++

This is **reliability**.

---

## Is Rust *fast*?<br>Is it *memory efficient*?

+++

Rust provides high-level features through what are called<br>**zero cost abstractions**.

+++

The features provided don't have a **run-time** cost, only a **compile time** cost.

+++

In other words, the abstractions are **as fast** and **as efficient** as one would write
the **same functionality by hand**.

+++

So you can use generics, type states, structs, vectors, the *newtype* pattern, etc., without worrying.

+++

At the end of the day, well-optimized **Rust code** can be **as fast as** well-optimized **C code**.

+++

So, high-level features with no run-time costs?

+++

This is **performance**.

---

## Going further

+++

Rust Book<br>https://doc.rust-lang.org/book/

+++

Rust Embedded Book<br>https://docs.rust-embedded.org/book/

+++

Rust by Example Book<br>https://doc.rust-lang.org/rust-by-example/

+++

## Other topics

+++

### Using cargo

To manage build types, test your code and cross compile.

+++

### The Type System

The *newtype*, the *type state* and the *builder* patterns.

+++

### Generics and Traits

Implementing shared behavior.

+++

### Enums

The Option and the Result type.

+++

### Memory Layout

Are Rust structs like C structs?

+++

### Parallel Programming

Threads, concurrency and Rayon.

---

## Questions?

---

## Reaching out to me

@jeandiegof on GitHub and Twitter<br>
jeanfontena@gmail.com

---

## Thank you for attending
