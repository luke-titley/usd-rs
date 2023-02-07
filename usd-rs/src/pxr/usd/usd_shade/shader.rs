use crate::pxr::usd::Prim;
//use crate::pxr::usd::attribute::*;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usdShade/shader.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct Shader as "pxr::UsdShadeShader");

impl Shader {
    pub fn new(prim: &Prim) -> Shader {
        unsafe {
            cpp!([prim as "pxr::UsdPrim*"]
                        -> Shader as "pxr::UsdShadeShader" {
                return pxr::UsdShadeShader::Get(prim->GetStage(), prim->GetPath());
            })
        }
    }

    pub fn get_prim(&self) -> Prim {
        unsafe {
            cpp!([self as "pxr::UsdShadeShader*"]
                        -> Prim as "pxr::UsdPrim" {
                return self->GetPrim();
            })
        }
    }
}
