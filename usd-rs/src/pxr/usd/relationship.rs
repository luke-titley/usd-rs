//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
//use crate::pxr::sdf;
use crate::pxr::sdf;
//use crate::pxr::tf;
//use crate::pxr::usd::attribute::*;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/relationship.h"
    #include <vector>
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct Relationship as "pxr::UsdRelationship");

impl Relationship {
    pub fn has_authored_targets(&self) -> bool {
        unsafe {
            cpp!([self as "pxr::UsdRelationship*"]
                        -> bool as "bool" {
                return self->HasAuthoredTargets();
            })
        }
    }

    pub fn get_targets(&self, targets: &mut sdf::PathVector) {
        let targets = targets.as_mut();
        unsafe {
            cpp!([self as "pxr::UsdRelationship*",
                  targets as "pxr::SdfPathVector*"] {
                self->GetTargets(targets);
            })
        }
    }
}
