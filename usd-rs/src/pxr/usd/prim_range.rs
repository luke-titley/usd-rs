//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/primRange.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
/// This allows for iterating over a collection of prims
#[repr(C, align(8))]
pub struct PrimRange {
    pub(in crate) _prim_range: *mut std::ffi::c_void,
    pub(in crate) _prim_range_it: *mut std::ffi::c_void,
}