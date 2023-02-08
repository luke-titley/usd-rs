//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//! # What is USD ?
//! USD is a system for authoring, composing, and reading hierarchically
//! organized scene description.
//!
//! USD comprises a set of modules that scalably encode and interchange static
//! and time-sampled 3D geometry and shading data between Digital Content
//! Creation applications. Domain-specific schema modules define the geometry
//! and shading encoding atop USD's domain-agnostic core.
//!
//! # What is usd-rs ?
//! usd-rs is a rust wrapper around the c++ implementation of USD. The goal is
//! to mirror the c++ api as closely as possible while still maintaining a 
//! certain degree of ergonmics.
//!
//! In order to be consistent with the namespaces and prefixes in the c++
//! library the root module is pxr, everything is a submodule of pxr.

#![deny(warnings)]
pub mod pxr;
