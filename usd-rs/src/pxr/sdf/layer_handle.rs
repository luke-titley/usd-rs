//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use crate::pxr;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #pragma GCC diagnostic ignored "-Wdeprecated-declarations"
    #pragma GCC diagnostic ignored "-Wdeprecated-copy"
    #include "pxr/usd/sdf/layer.h"
    #pragma GCC diagnostic pop
}}

cpp_class!(pub unsafe struct LayerHandle as "pxr::SdfLayerHandle");

impl LayerHandle {
    pub fn insert_sub_layer_path(
        &mut self,
        path: &str,
        index: Option<usize>,
    ) -> pxr::NoResult {
        let path_str = std::ffi::CString::new(path)?;
        let path = path_str.as_ptr() as *const std::os::raw::c_char;

        let index = if let Some(index) = index {
            index as i32
        } else {
            -1_i32
        };

        unsafe {
            cpp!([self as "pxr::SdfLayerHandle *",
                    path as "const char *",
                    index as "int"]{
                (*self)->InsertSubLayerPath(std::string(path), index);
            })
        };

        Ok(())
    }

    pub fn remove_sub_layer_path(&mut self, index: usize) {
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
