use crate::pxr::usd::attribute::*;
use crate::pxr::tf;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usdGeom/primvar.h"
    #include "pxr/usd/usdGeom/primvarsAPI.h"
    #include "pxr/usd/usdGeom/xformCache.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct Primvar as "pxr::UsdGeomPrimvar");

impl Primvar {
    pub fn new(attr: &Attribute) -> Primvar {
        unsafe {
            cpp!([attr as "pxr::UsdAttribute*"]
                        -> Primvar as "pxr::UsdGeomPrimvar" {
                return pxr::UsdGeomPrimvar(*attr);
            })
        }
    }

    pub fn get_interpolation(&self) -> tf::Token {
        unsafe {
            cpp!([self as "pxr::UsdGeomPrimvar*"]
                        -> tf::Token as "pxr::TfToken" {
                return self->GetInterpolation();
            })
        }
    }

}