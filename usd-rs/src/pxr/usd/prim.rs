//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
use crate::pxr::sdf;
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
pub struct AttributeDescriptor<'a> {
    pub name: tf::Token,
    pub _type_name: Option<&'a sdf::ValueTypeName>,
    //variability: Option<sdf::Variability>, // TODO
}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct Prim as "pxr::UsdPrim");

impl Prim {
    pub fn create_attribute(&self, _desc: AttributeDescriptor) {}
}
