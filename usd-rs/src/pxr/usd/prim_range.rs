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
pub(in crate) struct PrmRange {}

//------------------------------------------------------------------------------
/// This allows for iterating over a collection of prims
#[repr(C, align(8))]
pub struct PrimRange {
    pub(in crate) _prim_range: *const PrmRange,
}

impl Drop for PrimRange {
    fn drop(&mut self) {
        let prim_range = self._prim_range;
        unsafe {
            cpp!([prim_range as "pxr::UsdPrimRange *"] {
                delete prim_range;
            });
            self._prim_range = std::ptr::null_mut();
        };
    }
}
