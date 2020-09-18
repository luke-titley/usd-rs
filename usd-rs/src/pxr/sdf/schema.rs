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
// We dont care how big the Schema struct is, because we only ever use it as
// a reference. It's a singleton so the memory is all internal to usd.
#[repr(C, align(8))]
pub struct Schema {}

impl Schema {
    pub fn get_instance() -> &'static Schema {
        unsafe {
            cpp!([] -> & Schema as "const pxr::SdfSchema*" {
                return &pxr::SdfSchema::GetInstance();
            })
        }
    }

    pub fn find_type(&self, type_name: &tf::Token) -> ValueTypeName {
        unsafe {
            cpp!([self as "const pxr::SdfSchema*", type_name as "pxr::TfToken*"]
                -> ValueTypeName as "pxr::SdfValueTypeName" {
                return self->FindType(*type_name);
            })
        }
    }
}
