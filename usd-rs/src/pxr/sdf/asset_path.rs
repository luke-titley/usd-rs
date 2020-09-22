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
// Asset path can only be used accessed as a reference.
#[repr(C, align(8))]
pub struct AssetPath {
    // A private member stops users from being able to construct it without
    // Schema get_instance
    _priv: u8,
}

//------------------------------------------------------------------------------
pub struct BoxAssetPathDescriptor<'a> {
    pub path: &'a CStr,
    pub resolved_path: Option<&'a CStr>,
}

//------------------------------------------------------------------------------
#[repr(C, align(8))]
pub struct BoxAssetPath {
    _asset_path: *const AssetPath,
}

//------------------------------------------------------------------------------
impl BoxAssetPath {
    pub fn new(desc: BoxAssetPathDescriptor) -> Self {
        match desc {
            BoxAssetPathDescriptor {
                path,
                resolved_path: Some(resolved_path),
            } => unsafe {
                let path = path.as_ptr() as *const std::os::raw::c_char;
                let resolved_path =
                    resolved_path.as_ptr() as *const std::os::raw::c_char;

                cpp!([path as "const char *", resolved_path as "const char *"] -> BoxAssetPath as "const pxr::SdfAssetPath*" {
                    return new pxr::SdfAssetPath(std::string(path), std::string(resolved_path));
                })
            },
            BoxAssetPathDescriptor { path, .. } => unsafe {
                let path = path.as_ptr() as *const std::os::raw::c_char;

                cpp!([path as "const char *"] -> BoxAssetPath as "const pxr::SdfAssetPath*" {
                    return new pxr::SdfAssetPath(std::string(path));
                })
            },
        }
    }
}

//------------------------------------------------------------------------------
impl Drop for BoxAssetPath {
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
impl AsRef<AssetPath> for BoxAssetPath {
    fn as_ref(&self) -> &AssetPath {
        unsafe { &*(self._asset_path) }
    }
}
