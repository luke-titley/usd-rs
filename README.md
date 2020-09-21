# usd-rs

The very start of some rust bindings for [USD](https://github.com/PixarAnimationStudios/USD).

- usd-cpp is a crate to build the cpp shared library and its dependencies (tbb, boost ..).
- usd-rs is the actual bindings crate.
- usd-examples will be examples.

# Building
- git clone git@github.com:luke-titley/usd-rs.git && cd usd-rs
- git submodule update --init --recursive
- cargo build

# Requirements

- g++/clang with c++14 support
- cmake
- python

At the moment we use the handy 'build_usd.py' script that is bundled with USD to
download all the thirdparty libraries USD depends on. That's why we need python.
The USD library uses cmake, and obviously we are going to need a c++ compiler.

Perhaps in the future, it might be possible to port a subset of build_usd.py to
our build.rs and so remove the python dependency.

# The method
I'm using cpp crate for these bindings instead of cbindgen. At the time I tried out
cbindgen (16/9/2020) it was unable to parse USD without panicking. On top of that, cbindgen
produces a sys level crate which you use as a foundation for an ergonomic/safe hand written api.
The cpp crate allows you to write your safe api directly on top of the cpp library, and so
far it's been a good experience.

# Why not cxx crate? 'https://github.com/dtolnay/cxx'
I didn't know about it until I was a good way through.

# Adding a new basic type.
The attribute types supported by USD are finite and rarely change. However
there are a lot of them, so we employ code generation to implement them. This is
handled by the usd-basic-types crate, which is an executable for generating all
of the variants. We have to run this manually. All the types are listed in the
BASIC_TYPES constant a the top of usd-basic-types/src/main.rs

Add a new type to the constant BASIC_TYPES then run the below command.
```rust
cargo run --bin usd-basic-types > usd-rs/src/pxr/vt/basic_types.rs
```

## Why not use macros or build.rs?
We rely on the cpp! macro from rust-cpp, we cannot use this macro inside of
another macro.
