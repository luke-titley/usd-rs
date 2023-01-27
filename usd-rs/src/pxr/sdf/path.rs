//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use crate::pxr;
use cpp::*;

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

fn from_c_str(path: &std::ffi::CStr) -> Path {
    let path_str = path.as_ptr() as *const std::os::raw::c_char;

    unsafe {
        cpp!([path_str as "const char *"] -> Path as "pxr::SdfPath" {
            return pxr::SdfPath(std::string(path_str));
        })
    }
}

impl std::convert::TryFrom<&str> for Path {
    type Error = pxr::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let path_cstring = std::ffi::CString::new(value)?;

        Ok(from_c_str(path_cstring.as_c_str()))
    }
}
