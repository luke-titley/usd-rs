Rust bindings for [USD](https://github.com/PixarAnimationStudios/USD).

This is experimental and by no means complete.

# What works ?
- You can create and open/save/export a stage.
- You can define a prim and get/set attributes on it.
  All 30 basic types are supported.

# What doesn't work
Everything else.

# Things to note
This crate depends on usd-cpp. Which downloads the usd cpp lib and deps and builds it.
USD is a big library that relies heavily on templates, it can be very slow to
build. You just need to wait it out, once it's built, cargo will cache it and
you don't have to build it again.
