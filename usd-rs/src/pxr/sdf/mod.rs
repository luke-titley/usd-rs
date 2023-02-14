//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//! Sdf provides the foundations for serializing scene description to a
//! reference ascii format, or a (multitude of) plugin-defined format.  It also
//! provides the primitive abstractions for interacting with scene description,
//! such as SdfPath, SdfLayer, SdfPrimSpec.
//!
//! # Overview
//! Implements scene description **layers** in USD.  In USD, a complete scene
//! description is composed from partial scene description stored in SdfLayer
//! objects.  The primary unit of scene description within a layer is a **prim**
//! spec, represented by the SdfPrimSpec class.  A complete UsdPrim on a
//! stage is a composition of the prim's built-in fallback values and all of
//! the prim spec objects specified in Sdf layers.  (For an overview of prims
//! and stages, see the "Usd library overview".)
//!
//! Use methods on an SdfLayer object to export and save a layer to a file, or
//! to load a file from disk.  Scene description files are stored in '.usd'
//! format (one layer per file, ascii or binary).  Other features abstracted at
//! the layer level include undo/redo functionality and logging, which can be
//! customized by subclassing SdfLayerStateDelegateBase .
//!
//! You should primarily work with scene description using the classes in the
//! Usd library.  The UsdStage object not only represents a complete scene; it
//! also knows how each of the partial scene descriptions were combined to form
//! the complete scene.  For example, the UsdStage object has the context to
//! know how the path of a UsdPrim object on the stage relates to the paths of
//! each SdfPrimSpec object in each layer that contributes a partial
//! description to the complete prim.  SdfLayer objects do not have the context
//! to know how they relate to other layers.

mod asset_path;
mod layer_handle;
mod layer_handle_vector;
mod path;
mod path_vector;
mod schema;
mod time_code;
mod value_type_name;

//------------------------------------------------------------------------------
pub use asset_path::*;
pub use layer_handle::*;
pub use layer_handle_vector::*;
pub use path::*;
pub use path_vector::*;
pub use schema::*;
pub use time_code::*;
pub use value_type_name::*;
