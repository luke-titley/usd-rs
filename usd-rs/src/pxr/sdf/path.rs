//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use cpp::*;
use std::ffi::CStr;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/sdf/path.h"
    #pragma GCC diagnostic pop
}}

cpp_class!(pub unsafe struct Path as "pxr::SdfPath");

impl Path {
    pub fn get_text(&self) -> &std::ffi::CStr {
        unsafe {
            std::ffi::CStr::from_ptr(cpp!([self as "const pxr::SdfPath *"]
                    -> * const std::os::raw::c_char as "const char *" {
                return self->GetText();
            }))
        }
    }
}

impl From<&CStr> for Path {
    fn from(path: &CStr) -> Self {
        let path_str = path.as_ptr() as *const std::os::raw::c_char;

        unsafe {
            cpp!([path_str as "const char *"] -> Path as "pxr::SdfPath" {
                return pxr::SdfPath(std::string(path_str));
            })
        }
    }
}
