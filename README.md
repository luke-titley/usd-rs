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
