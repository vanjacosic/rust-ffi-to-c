# How to call a C function from Rust 🦀☎️

I was working on a Rust project where we needed to interact with code written in C.

I had to learn how to work with FFI (Foreign Function Interface) in Rust and wrote up this little guide for others.

This repository is a working example of the final code from the tutorial I wrote below. Clone it and run it using `cargo run`.

```bash
$ cargo run
   Compiling rust-ffi-to-c v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/rust-ffi-to-c`

[Rust] Hello from Rust! 🦀
[Rust] Calling function in C..
[C] Hello from C!
[C] Input a is: 5000
[C] Input b is: 5
[C] Multiplying and returning result to Rust..
[Rust] Result: 25000
```

## Tutorial

### 1. Define an external function

We use [`extern`](https://doc.rust-lang.org/reference/items/external-blocks.html) to reference the `multiply()` function, which is written in C (`src/multiply.c`).

In this case we want to multiply integers, so we import a C-compatible integer type into Rust from `core:ffi`. (See all the [available types](https://doc.rust-lang.org/core/ffi/index.html))

We then define the argument types and return type for our C function as `c_int` (equivalent to `i32` in Rust).

```rust
extern crate core;
use core::ffi::c_int;

extern "C" {
    fn multiply(a: c_int, b: c_int) -> c_int;
}
```

### 2. Call the C function from Rust

Any use of foreign function is considered unsafe because the Rust compiler can't guarantee memory safety in foreign code. 
So in our main Rust file (`src/main.rs`) we call the function in an `unsafe` block, then pass in two `i32` integers, and print the result.

```rust
unsafe {
    println!("Result: {}", multiply(5000, 5));
}
```

### 3. Compile and run

First we compile our `multiply.c` file using a C compiler:

    clang src/multiply.c -c

The `-c` flag tells the C compiler to output a "object file (`.o`)" instead of an executable program. So it creates a `multiply.o` file that we can use as a shared dynamic library in our Rust code.

Then we compile our program using the Rust compiler:

    rustc src/main.rs -l multiply.o -L .

The `-l multiply.o` option tells the Rust compiler to link the shared library.
The `-L .` option tells the Rust compiler to look for libraries in the current directory.

The compiler creates an executable named `main` which we can run:

    ./main
    Result: 25000

### 4. Automate 🤖

It gets tedious to compile the files manually every time, so we will use cargo build script and the [`cc`](https://crates.io/crates/cc) crate to automate this process.

Add `cc` to the projects build dependencies:

```toml
[build-dependencies]
cc = "1.0"
```

Create a `build.rs` and add compile instructions:

```rust
extern crate cc;

fn main() {
    cc::Build::new().file("src/multiply.c").compile("multiply");
}
```

And now we can use Cargo to build both the C and Rust code and run the program:

    cargo run


## Notes

- From [Rust 1.64.0](https://blog.rust-lang.org/2022/09/22/Rust-1.64.0.html#c-compatible-ffi-types-in-core-and-alloc) it is now recommended to use `core::ffi` instead of `std::os::raw` to access C types. The latter is now an alias to types in the `core::ffi` module. `core` is also available in places where the Rust standard library (`std`) is not, like [embedded projects](https://docs.rust-embedded.org/book/intro/no-std.html).

- Mapping out functions manully using `extern` is fine for small projects, but as soon as you are dealing with a bigger library or codebase, you want to take a look at `bindgen`. It can automatically generate the bindings for C or C++ libraries, making using them in Rust a lot easier. See [the `bindgen` User Guide](https://rust-lang.github.io/rust-bindgen/).

- We can control how our code is linked using the [`#[link()]` attribute.](https://doc.rust-lang.org/reference/items/external-blocks.html#the-link-attribute). It allows us to specify or rename functions and change the type of linking to use, eg. to static:

    ```rust
    #[link(name = "multiply", kind = "static")]
    extern "C" { // ... }
    ```

## Further reading

- [FFI chapter in The Rustonomicon book](https://doc.rust-lang.org/nomicon/ffi.html) (Rustonomicon is the official guide to unsafe Rust)

- [FFI chapter in the Secure Rust Guidelines book](https://anssi-fr.github.io/rust-guide/07_ffi.html)

- ["A little C with your Rust" chapter in The Embedded Rust Book](https://docs.rust-embedded.org/book/interoperability/c-with-rust.html) (The official Embedded Rust guide)

- [A Guide to Porting C and C++ code to Rust](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/)

- [Build Scripts - The Cargo Book](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

- [Deciphering Rust’s `#[no_mangle]` - pwnthebox.net](https://web.archive.org/web/20221113090341/https://www.pwnthebox.net/rust/2020/11/01/deciphering-no-mangle.html)

- [Rust FFI: Sending strings to the outside world | Huy's Blog](https://web.archive.org/web/20221007224430/https://snacky.blog/en/string-ffi-rust.html)

- [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)

- 📖 Chapter 11: *"Foreign Function Interfaces"* in [Rust for Rustaceans](https://nostarch.com/rust-rustaceans) by Jon Gjengset

- 📖 Chapter 23: *"Foreign Functions"* in [Programming Rust, 2nd Edition](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/) by Jim Blandy, Jason Orendorff & Leonora F. S. Tindall

## Further watching

- [Crust of Rust: Build Scripts and Foreign-Function Interfaces (FFI)](https://www.youtube.com/watch?v=pePqWoTnSmQ)
