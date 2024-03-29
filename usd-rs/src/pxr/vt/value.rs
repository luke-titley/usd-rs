//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use crate::pxr;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/base/vt/value.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(
    /// Provides a container which may hold any type, and provides introspection
    /// and iteration over array types.  See \a VtIsArray for more info.
    ///
    pub unsafe struct Value as "pxr::VtValue"
);

impl Value {
    pub fn new() -> Self {
        unsafe {
            cpp!([] -> Value as "pxr::VtValue" {
                return pxr::VtValue();
            })
        }
    }

    /// Returns a &str so long as the Value contains a Unicode
    /// string.
    pub fn try_as_ref(&self) -> pxr::Result<&str> {
        let result_cstr = unsafe {
            std::ffi::CStr::from_ptr(
                cpp!([self as "const pxr::VtValue *"] ->  * const std::os::raw::c_char as "const char *" {{
                    return self->Get<std::string>().c_str();
                }}),
            )
        };

        Ok(result_cstr.to_str()?)
    }
}
