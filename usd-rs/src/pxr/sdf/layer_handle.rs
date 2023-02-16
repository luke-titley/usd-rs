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

cpp_class!(
/// A scene description container that can combine with other such containers
/// to form simple component assets, and successively larger aggregates.  The
/// contents of an SdfLayer adhere to the SdfData data model.  A layer can be
/// ephemeral, or be an asset accessed and serialized through the ArAsset and
/// ArResolver interfaces.
    pub unsafe struct LayerHandle as "pxr::SdfLayerHandle"
);

impl LayerHandle {
    /// Inserts new sublayer path at the given index.
    ///
    /// The default index of -1 means to insert at the end.
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

    /// Removes sublayer path at the given index.
    pub fn remove_sub_layer_path(&mut self, index: usize) {
        let index = index as i32;

        unsafe {
            cpp!([self as "pxr::SdfLayerHandle *",
                    index as "int"]{
                (*self)->RemoveSubLayerPath(index);
            })
        };
    }

    /// Returns `true` if successful, `false` if an error occurred.
    /// Returns `false` if the layer has no remembered file name or the
    /// layer type cannot be saved. The layer will not be overwritten if the
    /// file exists and the layer is not dirty unless __force__ is true.
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
