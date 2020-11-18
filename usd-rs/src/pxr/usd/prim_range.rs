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

#[repr(C, align(8))]
struct PrimRangeImpl {}

//------------------------------------------------------------------------------
/// This is a reference to an asset path.
///
/// &AsstPth is to AssetPath, as &str is to String.
///
#[repr(C, align(8))]
pub struct PrimRange {
    _prim_range: *const PrimRangeImpl,
}