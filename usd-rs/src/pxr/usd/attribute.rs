//------------------------------------------------------------------------------
use crate::pxr::sdf;
use crate::pxr::tf;

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
