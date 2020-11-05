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
