//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/sdf/layer.h"
    #pragma GCC diagnostic pop
}}

cpp_class!(pub unsafe struct LayerHandle as "pxr::SdfLayerHandle");

impl LayerHandle {

    pub fn insert_sub_layer_path(& mut self, path : &std::ffi::CStr,
            index : Option<usize>) {

        let path = path.as_ptr() as *const std::os::raw::c_char;
        let index = if let Some(index) = index { index as i32 } else { -1_i32 };

        unsafe {
            cpp!([self as "pxr::SdfLayerHandle *",
                    path as "const char *",
                    index as "int"]{
                (*self)->InsertSubLayerPath(std::string(path), index);
            })
        };
    }

    pub fn remove_sub_layer_path(& mut self, index : usize) {
        let index = index as i32;

        unsafe {
            cpp!([self as "pxr::SdfLayerHandle *",
                    index as "int"]{
                (*self)->RemoveSubLayerPath(index);
            })
        };
    }

    pub fn save(&self, force: Option<bool>) -> bool {
        match force {
            Some(force) => unsafe {
                cpp!([self as "pxr::SdfLayerHandle *", force as "bool"]
                        -> bool as "bool" {
                    return (*self)->Save(force);
                })
            },
            None => unsafe {
                cpp!([self as "pxr::SdfLayerHandle *"] -> bool as "bool" {
                    return (*self)->Save();
                })
            },
        }
    }
}
