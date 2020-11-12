# usd-rs

Rust bindings for [USD](https://github.com/PixarAnimationStudios/USD).

These are in version 0.0.x. The api is likely to change significantly
between now and 0.1.0.

# What works ?
- You can create and open/save/export a stage.
- You can define a prim and get/set attributes on it.
  All 30 basic types are supported + arrays of those types.

# The parts
- usd-cpp is a crate to build the cpp shared library and its dependencies (tbb, boost ..).
- usd-rs is the actual bindings crate.

# Requirements

- g++/clang with c++14 support
- cmake
- python
- and of course cargo

