//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

use cpp::*;
use std::ffi::CStr;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/sdf/assetPath.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
/// This is a reference to an asset path.
///
/// &AsstPth is to AssetPath, as &str is to String.
///
#[repr(C, align(8))]
pub struct AsstPth {
    // A private member stops users from being able to construct it without
    // Schema get_instance
    _priv: u8,
}

//------------------------------------------------------------------------------
// AssetPathDescriptor
//------------------------------------------------------------------------------
pub struct AssetPathDescriptor<'a> {
    pub path: &'a CStr,
    pub resolved_path: Option<&'a CStr>,
}

//------------------------------------------------------------------------------
#[repr(C, align(8))]
pub struct AssetPath {
    _asset_path: *const AsstPth,
}

//------------------------------------------------------------------------------
impl AssetPath {
    pub fn new(desc: AssetPathDescriptor) -> Self {
        match desc {
            AssetPathDescriptor {
                path,
                resolved_path: Some(resolved_path),
            } => unsafe {
                let path = path.as_ptr() as *const std::os::raw::c_char;
                let resolved_path =
                    resolved_path.as_ptr() as *const std::os::raw::c_char;

                cpp!([path as "const char *", resolved_path as "const char *"] -> AssetPath as "const pxr::SdfAssetPath*" {
                    return new pxr::SdfAssetPath(std::string(path), std::string(resolved_path));
                })
            },
            AssetPathDescriptor { path, .. } => unsafe {
                let path = path.as_ptr() as *const std::os::raw::c_char;

                cpp!([path as "const char *"] -> AssetPath as "const pxr::SdfAssetPath*" {
                    return new pxr::SdfAssetPath(std::string(path));
                })
            },
        }
    }
}

//------------------------------------------------------------------------------
impl Drop for AssetPath {
    fn drop(&mut self) {
        let asset_path = self._asset_path.clone();
        unsafe {
            cpp!([asset_path as "const pxr::SdfAssetPath*"] {
                delete asset_path;
            })
        }
    }
}

//------------------------------------------------------------------------------
impl AsRef<AsstPth> for AssetPath {
    fn as_ref(&self) -> &AsstPth {
        unsafe { &*(self._asset_path) }
    }
}
