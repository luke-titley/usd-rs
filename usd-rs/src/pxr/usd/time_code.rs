//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
//use crate::pxr::sdf;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/timeCode.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct TimeCode as "pxr::UsdTimeCode");

impl From<f64> for TimeCode {
    fn from(t : f64) -> Self {
        unsafe {
            cpp!([t as "double"] -> TimeCode as "pxr::UsdTimeCode" {
                return pxr::UsdTimeCode(t);
            })
        }
    }
}