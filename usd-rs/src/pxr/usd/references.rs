//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

use cpp::*;
use crate::pxr::sdf;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/references.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
pub mod desc {
    use super::*;

    pub struct AddReference<'a> {
        pub identifier: &'a std::ffi::CStr,
        pub prim_path: Option<&'a sdf::Path>,
    }
}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct References as "pxr::UsdReferences");

impl References {
    pub fn add_reference(&mut self, desc : desc::AddReference) -> bool {
        match desc {
            desc::AddReference {
                identifier,
                prim_path: None,
            } => {
                let identifier =
                    identifier.as_ptr() as *const std::os::raw::c_char;

                unsafe {
                    cpp!([self as "pxr::UsdReferences*",
                          identifier as "const char *"
                    ] -> bool as "bool" {
                        return self->AddReference(std::string(identifier));
                    })
                }
            },
            desc::AddReference {
                identifier,
                prim_path: Some(prim_path),
            } => {
                let identifier =
                    identifier.as_ptr() as *const std::os::raw::c_char;

                unsafe {
                    cpp!([self as "pxr::UsdReferences*",
                          identifier as "const char *",
                          prim_path as "const pxr::SdfPath *"
                    ] -> bool as "bool" {
                        return self->AddReference(std::string(identifier),
                                                  *prim_path);
                    })
                }
            }
        }
    }

    pub fn clear_references() -> bool {
        false
    }
}