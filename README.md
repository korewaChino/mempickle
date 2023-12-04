# mempickle

> :warning: **WARNING**: This crate should **NEVER** be used in production environments. All your data will be corrupted if you EVER change your code, or even recompile it with a different compiler version. This crate is only meant for learning purposes.

This crate provides a `MemPickle` type that can pickle raw Rust structs into a byte buffer and unpickle them back.

I wrote this crate just to learn how to work with raw pointers in Rust. If you insist on using it, it's your funeral.

Load with feature "i_like_breaking_production" if you insist on using this crate.

## Usage

```rust
use mempickle::MemPickle;

#[derive(Debug, PartialEq)]
struct Foo {
    a: u32,
    b: u32,
}

let foo = Foo { a: 42, b: 69 };

let pickle = MemPickle::new(foo);


// do some other stuff until you need to load the pickle back

let foo = pickle.unwrap();

assert_eq!(foo, Foo { a: 42, b: 69 });
```
