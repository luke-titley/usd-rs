//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/sdf/path.h"
    #pragma GCC diagnostic pop
}}

cpp_class!(pub unsafe struct Path as "pxr::SdfPath");

impl From<&str> for Path {
    fn from(path: &str) -> Self {
        let path = std::ffi::CString::new(path)
            .expect("Unable to convert path to CString");

        let path_str = path.as_ptr() as *const std::os::raw::c_char;

        unsafe {
            cpp!([path_str as "const char *"] -> Path as "pxr::SdfPath" {
                return pxr::SdfPath(std::string(path_str));
            })
        }
    }
}
