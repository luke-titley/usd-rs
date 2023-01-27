//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use crate::pxr;
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
impl AsstPth {
    pub fn get_asset_path(&self) -> &CStr {
        use std::os::raw::c_char;
        unsafe {
            CStr::from_ptr(
                cpp!([self as "const pxr::SdfAssetPath*"] ->  * const c_char as "const char *" {
                    return self->GetAssetPath().c_str();
                }),
            )
        }
    }
    pub fn get_resolved_path(&self) -> &CStr {
        use std::os::raw::c_char;
        unsafe {
            CStr::from_ptr(
                cpp!([self as "const pxr::SdfAssetPath*"] ->  * const c_char as "const char *" {
                    return self->GetResolvedPath().c_str();
                }),
            )
        }
    }
}

//------------------------------------------------------------------------------
// AssetPathDescriptor
//------------------------------------------------------------------------------
pub struct AssetPathDescriptor<'a> {
    pub path: &'a str,
    pub resolved_path: Option<&'a str>,
}

//------------------------------------------------------------------------------
#[repr(C, align(8))]
pub struct AssetPath {
    _asset_path: *const AsstPth,
}

//------------------------------------------------------------------------------
impl AssetPath {
    pub fn new(desc: AssetPathDescriptor) -> pxr::Result<Self> {
        Ok(match desc {
            AssetPathDescriptor {
                path,
                resolved_path: Some(resolved_path),
            } => unsafe {
                let path = std::ffi::CString::new(path)?;
                let resolved_path = std::ffi::CString::new(resolved_path)?;

                let path_c_char = path.as_ptr() as *const std::os::raw::c_char;
                let resolved_path_c_char =
                    resolved_path.as_ptr() as *const std::os::raw::c_char;

                cpp!([path_c_char as "const char *", resolved_path_c_char as "const char *"]
                        -> AssetPath as "const pxr::SdfAssetPath*" {
                    return new pxr::SdfAssetPath(std::string(path_c_char), std::string(resolved_path_c_char));
                })
            },
            AssetPathDescriptor { path, .. } => unsafe {
                let path = std::ffi::CString::new(path)?;
                let path_c_char = path.as_ptr() as *const std::os::raw::c_char;

                cpp!([path_c_char as "const char *"]
                        -> AssetPath as "const pxr::SdfAssetPath*" {
                    return new pxr::SdfAssetPath(std::string(path_c_char));
                })
            },
        })
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
