# usd-rs

Rust bindings for [USD](https://github.com/PixarAnimationStudios/USD).


| Usd Version | API Documentation                                           | CI Status                                                                       |
| ----------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------- |
| 23.02       | [Docs](https://luke-titley.github.io/usd-rs/usd/index.html) | ![CI](https://github.com/luke-titley/usd-rs/actions/workflows/ci.yml/badge.svg) |


# USD version: 23.02

# What works ?
- You can create and open/save/export a stage.
- You can define a prim and get/set attributes on it.
  All 30 basic types are supported + arrays of those types.
- You can create references under prims or as layers.
- You can traverse the prims in a stage.

# Work to do
- [ ] Relationships
- [ ] Schemas
- [ ] Edit Targets
- [ ] Variant Sets
- [ ] Get/Set default root prim

# The parts
- usd-cpp is a crate to build the cpp shared library and its dependencies (tbb, boost ..).
- usd-rs is the actual bindings crate.
- usd-examples will be examples.

# Building
Before you do anything in terms of development it's work noting that there is a [docker](#Docker) image.
- git clone git@github.com:luke-titley/usd-rs.git && cd usd-rs
- git submodule update --init --recursive
- cargo build

## Building against your own USD build
Same as above only instead of cargo build do:
- env USD_ROOT=<path to your usd install> cargo build

# Running tests
One you've got the project building you can run the tests with
> cargo test
or
> env USD_ROOT=<path to your usd install> LD_LIBRARY_PATH=<path to your usd install>/lib cargo test


# Requirements

- g++/clang with c++14 support
- cmake
- python
- and of course cargo

At the moment we use the handy 'build_usd.py' script that is bundled with USD to
download all the thirdparty libraries USD depends on. That's why we need python.
The USD library uses cmake, and obviously we are going to need a c++ compiler.

Perhaps in the future, it might be possible to port a subset of build_usd.py to
our build.rs and so remove the python dependency.

# The method
I'm using cpp crate for these bindings in the mid term. This makes it easy to
hand write the wrappers and work the api. Work on automatically generated
bindings is going on in the background. Ultimately this crate will move over
to using usd-sys, when that work is complete.

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
another macro. While developing usd-rs its important to be able to see the types
inside of the module, it make it much easier.

## Docker
There is a pre-built docker image for those who have docker installed.
This image is intended for the ci to test usd-rs. It contains the environment
for building usd-rs in centos:7 which is the oldest supported linux distribution
across the vfx industry.

The image is called [luketitley/vfxrs_env_usd](https://hub.docker.com/repository/docker/luketitley/vfxrs_env_usd).
