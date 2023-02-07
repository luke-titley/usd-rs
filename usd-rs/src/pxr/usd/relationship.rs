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
/* pub mod desc {
    use super::*;
    use crate::pxr::sdf;

    pub struct CreateAttribute {
        pub name: tf::Token,
        pub type_name: sdf::ValueTypeName,
        //variability: Option<sdf::Variability>, // TODO
    }
} */

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

    pub fn get_target(&self) -> sdf::Path {
        unsafe {
            cpp!([self as "pxr::UsdRelationship*"]
                        -> sdf::Path as "pxr::SdfPath" {
                pxr::SdfPathVector targets;
                self->GetTargets(&targets);
                return targets[0];
            })
        }
    }

}
