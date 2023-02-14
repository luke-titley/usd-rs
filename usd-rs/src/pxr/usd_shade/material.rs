use crate::pxr::usd::Attribute;
use crate::pxr::usd::Prim;
use crate::pxr::usd_shade::shader::*;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usdShade/material.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct Material as "pxr::UsdShadeMaterial");

impl Material {
    pub fn new(prim: &Prim) -> Material {
        unsafe {
            cpp!([prim as "pxr::UsdPrim*"]
                        -> Material as "pxr::UsdShadeMaterial" {
                return pxr::UsdShadeMaterial::Get(prim->GetStage(), prim->GetPath());
            })
        }
    }

    pub fn get_surface_attribute(&self) -> Attribute {
        unsafe {
            cpp!([self as "pxr::UsdShadeMaterial*"]
                        -> Attribute as "pxr::UsdAttribute" {
                return self->GetSurfaceAttr();
            })
        }
    }

    pub fn compute_surface_source(&self) -> Shader {
        unsafe {
            cpp!([self as "pxr::UsdShadeMaterial*"]
                        -> Shader as "pxr::UsdShadeShader" {
                return self->ComputeSurfaceSource();
            })
        }
    }
}
