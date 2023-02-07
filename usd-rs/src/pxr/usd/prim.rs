//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
//use crate::pxr::sdf;
use crate::pxr::sdf;
use crate::pxr::tf;
use crate::pxr::usd::attribute::*;
use crate::pxr::usd::references::References;
use crate::pxr::usd::relationship::Relationship;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/prim.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
pub mod desc {
    use super::*;
    use crate::pxr::sdf;

    pub struct CreateAttribute {
        pub name: tf::Token,
        pub type_name: sdf::ValueTypeName,
        //variability: Option<sdf::Variability>, // TODO
    }
}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct Prim as "pxr::UsdPrim");

impl Prim {
    pub fn get_type_name(&self) -> &tf::Token {
        unsafe {
            cpp!([self as "const pxr::UsdPrim*"]
                        -> * const tf::Token as "const pxr::TfToken*" {
                return &self->GetTypeName();
            })
            .as_ref()
            .unwrap()
        }
    }

    pub fn get_references(&self) -> References {
        unsafe {
            cpp!([self as "const pxr::UsdPrim*"]
                        -> References as "const pxr::UsdReferences" {
                return self->GetReferences();
            })
        }
    }

    pub fn get_relationship(&self, rel_name: &tf::Token) -> Relationship {
        unsafe {
            cpp!([self as "const pxr::UsdPrim*",
                rel_name as "pxr::TfToken*"]
                        -> Relationship as "const pxr::UsdRelationship" {
                return self->GetRelationship(*rel_name);
            })
        }
    }

    pub fn has_attribute(&self, attr_name: &tf::Token) -> bool {
        unsafe {
            cpp!([self as "pxr::UsdPrim*",
                attr_name as "pxr::TfToken*"]
                        -> bool as "bool" {
                return self->HasAttribute(*attr_name);
            })
        }
    }

    pub fn has_relationship(&self, rel_name: &tf::Token) -> bool {
        unsafe {
            cpp!([self as "const pxr::UsdPrim*",
                rel_name as "pxr::TfToken*"]
                        -> bool as "bool" {
                return self->HasRelationship(*rel_name);
            })
        }
    }

    pub fn create_attribute(&self, desc: desc::CreateAttribute) -> Attribute {
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

    pub fn get_attribute(&self, attr_name: &tf::Token) -> Attribute {
        unsafe {
            cpp!([self as "pxr::UsdPrim*",
                attr_name as "pxr::TfToken*"]
                        -> Attribute as "pxr::UsdAttribute" {
                return self->GetAttribute(*attr_name);
            })
        }
    }

    // this was causing segfault, hence the dirty implementation below.
    /*     pub fn get_attributes(&self) -> Vec<Attribute> {
        unsafe {
            cpp!([self as "pxr::UsdPrim*"]
                        -> Vec<Attribute> as "std::vector<pxr::UsdAttribute>" {
                return self->GetAttributes();
            })
        }
    } */

    pub fn get_attributes(&self) -> Vec<Attribute> {
        let mut out: Vec<Attribute> = Vec::new();
        let n = unsafe {
            cpp!([self as "pxr::UsdPrim*"]
                        -> i32 as "int" {
                return self->GetAttributes().size();
            })
        };
        for i in 0..n {
            let attr = unsafe {
                cpp!([self as "pxr::UsdPrim*",
                    i as "int"]
                    -> Attribute as "pxr::UsdAttribute" {
                    return self->GetAttributes()[i];
                })
            };
            out.push(attr);
        }
        out
    }

    pub fn get_name(&self) -> &tf::Token {
        unsafe {
            cpp!([self as "const pxr::UsdPrim*"]
                        -> * const tf::Token as "const pxr::TfToken*" {
                return &self->GetName();
            })
            .as_ref()
            .unwrap()
        }
    }

    pub fn get_path(&self) -> sdf::Path {
        unsafe {
            cpp!([self as "const pxr::UsdPrim*"]
                        -> sdf::Path as "pxr::SdfPath" {
                return self->GetPath();
            })
        }
    }
}
