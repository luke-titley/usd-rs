//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//! USD is a system for authoring, composing, and reading hierarchically
//! organized scene description.
//! USD comprises a set of modules that scalably encode and interchange static
//! and time-sampled 3D geometry and shading data between Digital Content
//! Creation applications. Domain-specific schema modules define the geometry
//! and shading encoding atop USD's domain-agnostic core.

#![deny(warnings)]
pub mod pxr;

pub use c_str_macro::c_str;
