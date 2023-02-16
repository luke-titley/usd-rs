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
/// This is a reference to an AssetPath.
///
/// &AssetPathRef is to [AssetPath], as &str is to String.
///
#[repr(C, align(8))]
pub struct AssetPathRef {
    // A private member stops users from being able to construct it without
    // Schema get_instance
    _priv: u8,
}

//------------------------------------------------------------------------------
impl AssetPathRef {
    /// Return the asset path as a &str
    pub fn get_asset_path(&self) -> pxr::Result<&str> {
        use std::os::raw::c_char;

        let result = unsafe {
            CStr::from_ptr(
                cpp!([self as "const pxr::SdfAssetPath*"] ->  * const c_char as "const char *" {
                    return self->GetAssetPath().c_str();
                }),
            )
        };

        Ok(result.to_str()?)
    }

    /// Return the resolved asset path(as a &str) if any.
    ///
    /// Note that [AssetPath] carries a resolved path only if its creator
    /// passed one to the constructor.  [AssetPath] never performs resolution
    /// itself.
    pub fn get_resolved_path(&self) -> pxr::Result<&str> {
        use std::os::raw::c_char;

        let result = unsafe {
            CStr::from_ptr(
                cpp!([self as "const pxr::SdfAssetPath*"] ->  * const c_char as "const char *" {
                    return self->GetResolvedPath().c_str();
                }),
            )
        };

        Ok(result.to_str()?)
    }
}

impl std::fmt::Display for AssetPathRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(text) = self.get_asset_path() {
            write!(f, "{}", text)
        } else {
            write!(f, "<invalid path>")
        }
    }
}

//------------------------------------------------------------------------------
// AssetPathDescriptor
//------------------------------------------------------------------------------
pub struct AssetPathDescriptor<'a> {
    /// The path to hold
    pub path: &'a str,
    /// The resolved path, this is optional
    pub resolved_path: Option<&'a str>,
}

//------------------------------------------------------------------------------
/// Contains an asset path and an optional resolved path.  Asset paths may
/// contain non-control UTF-8 encoded characters.  Specifically, U+0000..U+001F
/// (C0 controls), U+007F (delete), and U+0080..U+009F (C1 controls) are
/// disallowed.  Attempts to construct asset paths with such characters will
/// issue a TfError and produce the default-constructed empty asset path.
#[repr(C, align(8))]
pub struct AssetPath {
    _asset_path: *const AssetPathRef,
}

//------------------------------------------------------------------------------
impl AssetPath {
    /// Construct an asset path with `path` and an associated `resolvedPath`.
    ///
    /// If either the passed `path` or `resolvedPath` are not valid UTF-8 or
    /// either contain C0 or C1 control characters, raise a TfError and return
    /// the default-constructed empty asset path.
    ///
    /// The parameters are in [AssetPathDescriptor].
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
impl AsRef<AssetPathRef> for AssetPath {
    fn as_ref(&self) -> &AssetPathRef {
        unsafe { &*(self._asset_path) }
    }
}

//------------------------------------------------------------------------------
impl std::ops::Deref for AssetPath {
    type Target = AssetPathRef;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
