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
/// Descriptors for construction of new data on a reference
pub mod references_desc {
    use super::*;

    /// Parameters for adding a new reference
    pub struct AddReference<'a> {
        pub identifier: &'a str,
        pub prim_path: Option<&'a sdf::Path>,
    }
}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct References as "pxr::UsdReferences");

/// UsdReferences provides an interface to authoring and introspecting
/// references in Usd.
///
/// References are the primary operator for "encapsulated aggregation" of
/// scene description.  __aggregation__ means that references let us
/// build up rich scenes by composing scene description recorded in a (most
/// often) different layer.  A scene can reference the same layer many times at
/// different locations in a scene's namespace.  Referenced scene description
/// can be overridden in the referencing (or stronger) layers, allowing each
/// instance of the reference to be directly customized/overridden.
impl References {
    /// Adds a reference to the reference listOp at the current EditTarget.
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

    /// Removes the authored reference listOp edits at the current EditTarget.
    /// The same caveats for remove() apply to clear().  In fact, clear() may
    /// actually increase the number of composed references, if the listOp
    /// being cleared contained the "remove" operator.
    pub fn clear_references(&mut self) -> bool {
        unsafe {
            cpp!([self as "pxr::UsdReferences*"] -> bool as "bool" {
                return self->ClearReferences();
            })
        }
    }
}
