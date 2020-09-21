//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
// AUTOMATICALLY GENERATED : Don't edit by hand.
// See usd-basic-types

use super::Value;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/base/vt/value.h"
    #pragma GCC diagnostic pop
}}

impl From<&bool> for Value {
    fn from(other: &bool) -> Self {
        unsafe {
            cpp!([other as "const bool *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<bool> for Value {
    fn as_ref(&self) -> &bool {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &bool as "const bool *" {
                return &(self->Get<bool>());
            })
        }
    }
}
