//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//! Defines classes that provide for type abstraction (VtValue) and enhanced
//! array types (VtArray).
//!
//! The Vt library also provides functions for manipulating value types.
//! This library operates on the level of language data types and there are
//! differences in the C++ and Rust interfaces.
//!
//! # vt_value Type Erasure with VtValue
//! The VtValue class wraps type objects (float, int, bool, GfVec3d, and so on)
//! in a type-agnostic container that includes functions for determining the
//! content type within the container.  The VtValue class is found in the C++
//! and Rust API only.
//!
//! # vt_array Shared Arrays - VtArray
//! The VtArray class represents an arbitrary length homogeneous container.  In
//! the C++ API, the constructor lets you create an array of a specified size.  
//! The VtArray interface on the Rust side is implemented as a set of typed
//! array classes (for example, ArrayBool, ArrayString, ArrayVec4d).

mod array;
mod basic_types;
mod value;

//------------------------------------------------------------------------------
pub use array::*;
pub use basic_types::*;
pub use value::*;
