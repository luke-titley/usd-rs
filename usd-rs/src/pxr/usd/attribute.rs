//------------------------------------------------------------------------------
use crate::pxr::sdf;
use crate::pxr::tf;
use crate::pxr::usd::TimeCode;
use crate::pxr::vt;

use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/attribute.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
pub struct AttributeDescriptor {
    pub name: tf::Token,
    pub type_name: sdf::ValueTypeName,
    //variability: Option<sdf::Variability>, // TODO
}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct Attribute as "pxr::UsdAttribute");

impl Attribute {
    pub fn set(&self, value: &vt::Value, time: TimeCode) {
        unsafe {
            cpp!([self as "const pxr::UsdAttribute *",
                  value as "const pxr::VtValue*",
                  time as "pxr::UsdTimeCode"] {
                self->Set(*value, time);
            })
        }
    }

    pub fn get(&self, value: &mut vt::Value, time: TimeCode) {
        unsafe {
            cpp!([self as "const pxr::UsdAttribute *",
                  value as "pxr::VtValue*",
                  time as "pxr::UsdTimeCode"] {
                self->Get(value, time);
            })
        }
    }
}
