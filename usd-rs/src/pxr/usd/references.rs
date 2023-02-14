//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use crate::pxr;
use crate::pxr::sdf;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/references.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
pub mod references_desc {
    use super::*;

    pub struct AddReference<'a> {
        pub identifier: &'a str,
        pub prim_path: Option<&'a sdf::Path>,
    }
}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct References as "pxr::UsdReferences");

impl References {
    pub fn add_reference(
        &mut self,
        desc: references_desc::AddReference,
    ) -> pxr::NoResult {
        let result = match desc {
            references_desc::AddReference {
                identifier,
                prim_path: None,
            } => {
                let identifier_cstring = std::ffi::CString::new(identifier)?;
                let identifier =
                    identifier_cstring.as_ptr() as *const std::os::raw::c_char;

                let result = unsafe {
                    cpp!([self as "pxr::UsdReferences*",
                        identifier as "const char *"
                    ] -> bool as "bool" {
                        return self->AddReference(std::string(identifier));
                    })
                };
                result
            }
            references_desc::AddReference {
                identifier,
                prim_path: Some(prim_path),
            } => {
                let identifier_cstring = std::ffi::CString::new(identifier)?;
                let identifier =
                    identifier_cstring.as_ptr() as *const std::os::raw::c_char;

                let result = unsafe {
                    cpp!([self as "pxr::UsdReferences*",
                        identifier as "const char *",
                        prim_path as "const pxr::SdfPath *"
                    ] -> bool as "bool" {
                        return self->AddReference(std::string(identifier),
                                                  *prim_path);
                    })
                };
                result
            }
        };

        if result {
            Ok(())
        } else {
            Err(pxr::Error::UnableToAddReference)
        }
    }

    pub fn clear_references(&mut self) -> bool {
        unsafe {
            cpp!([self as "pxr::UsdReferences*"] -> bool as "bool" {
                return self->ClearReferences();
            })
        }
    }
}
