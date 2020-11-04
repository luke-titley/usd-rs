//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
//use crate::pxr::sdf;
use crate::pxr::usd::attribute::*;
use crate::pxr::tf;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/prim.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct Prim as "pxr::UsdPrim");

impl Prim {
    pub fn create_attribute(&self, desc: AttributeDescriptor) -> Attribute {
        let name = &desc.name;
        let type_name = &desc.type_name;

        unsafe {
            cpp!([self as "pxr::UsdPrim*",
                  name as "pxr::TfToken*",
                  type_name as "pxr::SdfValueTypeName*"]
                        -> Attribute as "pxr::UsdAttribute" {
                return self->CreateAttribute(*name, *type_name);
            })
        }
    }

    pub fn get_attribute(&self, attr_name : &tf::Token) -> Attribute {
        unsafe {
            cpp!([self as "pxr::UsdPrim*",
                attr_name as "pxr::TfToken*"]
                        -> Attribute as "pxr::UsdAttribute" {
                return self->GetAttribute(attr_name);
            })
        }
    }
}
