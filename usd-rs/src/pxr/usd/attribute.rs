//------------------------------------------------------------------------------
use crate::pxr;
use crate::pxr::tf;
use crate::pxr::usd::TimeCode;
use crate::pxr::vt;

use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #pragma GCC diagnostic ignored "-Wdeprecated-copy"
    #pragma GCC diagnostic ignored "-Wdeprecated-declarations"
    #include "pxr/usd/usd/attribute.h"
    #pragma GCC diagnostic pop
}}

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

    pub fn get_name(&self) -> pxr::Result<&tf::Token> {
        unsafe {
            let token_ptr = cpp!([self as "const pxr::UsdAttribute *"]
                        -> * const tf::Token as "const pxr::TfToken*" {
                return &self->GetName();
            });
            token_ptr
                .as_ref()
                .ok_or(pxr::Error::UnableToDereferencePointer)
        }
    }

    pub fn get_type_name(&self) -> pxr::sdf::ValueTypeName {
        unsafe {
            cpp!([self as "const pxr::UsdAttribute *"]
                        -> pxr::sdf::ValueTypeName as "pxr::SdfValueTypeName" {
                return self->GetTypeName();
            })
        }
    }

    pub fn has_value(&self) -> bool {
        unsafe {
            cpp!([self as "const pxr::UsdAttribute *"]
                        -> bool as "bool" {
                return self->HasValue();
            })
        }
    }

    pub fn get_metadata(&self, key: &tf::Token, value: &mut vt::Value) {
        unsafe {
            cpp!([self as "const pxr::UsdAttribute *",
                key as "pxr::TfToken",
                value as "pxr::VtValue*"] {
                self->GetMetadata(key, value);
            })
        }
    }
}
