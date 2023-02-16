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
    #include "pxr/usd/sdf/timeCode.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(
    /// Value type that represents a time code. It's equivalent to a double type
    /// value but is used to indicate that this value should be resolved by any
    /// time based value resolution.
    ///
    pub unsafe struct TimeCode as "pxr::SdfTimeCode"
);

impl From<f64> for TimeCode {
    fn from(t: f64) -> Self {
        unsafe {
            cpp!([t as "double"] -> TimeCode as "pxr::SdfTimeCode" {
                return pxr::SdfTimeCode(t);
            })
        }
    }
}
