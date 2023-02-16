//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use crate::pxr;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/sdf/valueTypeName.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct ValueTypeName as "pxr::SdfValueTypeName");

impl ValueTypeName {
    pub fn get_as_token(&self) -> pxr::tf::Token {
        unsafe {
            cpp!([self as "const pxr::SdfValueTypeName *"]
                        -> pxr::tf::Token as "pxr::TfToken" {
                return self->GetAsToken();
            })
        }
    }
}

impl std::fmt::Display for ValueTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(text) = self.get_as_token().get_text() {
            write!(f, "{}", text)
        } else {
            write!(f, "<invalid value type name>")
        }
    }
}
