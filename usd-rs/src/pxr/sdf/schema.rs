//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use super::value_type_name::ValueTypeName;
use crate::pxr::tf;

use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/sdf/schema.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
/// Provides information about the various scene description fields.
///
#[repr(C, align(8))]
pub struct Schema {
    // A private member stops users from being able to construct it without
    // Schema get_instance
    _priv: u8,
}

impl Schema {
    /// Schema is a singleton. This retrieves it.
    pub fn get_instance() -> &'static Schema {
        unsafe {
            cpp!([] -> & Schema as "const pxr::SdfSchema*" {
                return &pxr::SdfSchema::GetInstance();
            })
        }
    }

    /// Return the type name object for the given type name token.
    pub fn find_type(&self, type_name: &tf::Token) -> ValueTypeName {
        unsafe {
            cpp!([self as "const pxr::SdfSchema*", type_name as "pxr::TfToken*"]
                -> ValueTypeName as "pxr::SdfValueTypeName" {
                return self->FindType(*type_name);
            })
        }
    }
}
