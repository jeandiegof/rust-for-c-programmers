# Plan

1 - What is Rust?
    - Programming language
    - Compiled, no garbage collector
    - Uses LLVM for code generation (that's important)
    - Mix of high level features and low-level performance (iterators, good standard library, async support)
    - May or may not be an OOP, depending on your definition
        - has "objects": data + operations (adti, structs and enums)
        - has encapsulation: private members, access through well-defined api
        - does not have inheritance: thank god
            - code reuse -> traits for common behaviours
            - polymorphism -> bounded parametric polymorphism
        

2 - Who is using Rust?
    - firefox, discord, amazon, dropbox
    - support for Rust modules added to the linux kernel

3 - Why Rust?
    - from the website: performance, reliability, productivity
    - performance: memory efficient and safe
    - reliability: type system and ownership model
    - productivity: a good toolchain

4 - Installation
    - website https://www.rust-lang.org/tools/install

5 - productivity: toolchain (cargo, Cargo.toml, crates, cargo test, cross compilation)
    - add
    - add that works: add + tests
    - colorful add that works: add + tests + ansi_term
    - future: colorful add that works everywhere: add + tests + ansi_term + cross

6 - reliability: type system and ownership model
    - strongly typed:
        - u32 cannot be used where a u8 is expected
    - ownership:
        - other languages: alloc/free, garbage collector
        - rust: ownership -> set of rules checked at compile time (no runtime costs)
                goal: manage the heap efficiently

7 - But seriously, why Rust? (Examples)
    a) Immutable by default (friendly error messages)
    b) Ownership (the borrow checker)
    c) Single &mut, multiple &
    d) Lifetimes
    e) Powerful enums
    f) Powerful matching
    g) Type system: state machine
    h) Option<T>
    i) Result<T, E> (goodby early returns)
    j) Iterator expressiveness
    k) Traits

8 - Extras
    - Rust for embedded (microbit, cortex-m*)
    - Expressiveness + traits
    - Templates with monomorphization
    - Parallel programming? Rayon?
