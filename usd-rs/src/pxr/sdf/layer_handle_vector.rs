//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use super::layer_handle::LayerHandle;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/sdf/layerUtils.h"
    #pragma GCC diagnostic pop
}}

cpp_class!(pub unsafe struct LayerHandleVector as "pxr::SdfLayerHandleVector");

impl LayerHandleVector {
    pub fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::SdfLayerHandleVector *"]
                -> usize as "size_t" {
                return self->size();
            })
        }
    }

    pub fn reserve(&mut self, num: usize) {
        unsafe {
            cpp!([self as "pxr::SdfLayerHandleVector *",
                  num as "size_t"] {
                self->reserve(num);
            })
        }
    }

    pub fn push_back(&mut self, elem: &LayerHandle) {
        unsafe {
            cpp!([self as "pxr::SdfLayerHandleVector *",
                  elem as "const pxr::SdfLayerHandle *"] {
                self->push_back(*elem);
            })
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            cpp!([self as "pxr::SdfLayerHandleVector *"] {
                self->clear();
            })
        }
    }
}
