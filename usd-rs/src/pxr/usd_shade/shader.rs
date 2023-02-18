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
cpp_class!(
/// Base class for all USD shaders. Shaders are the building blocks
/// of shading networks. While UsdShadeShader objects are not target specific,
/// each renderer or application target may derive its own renderer-specific
/// shader object types from this base, if needed.
///
/// Objects of this class generally represent a single shading object, whether
/// it exists in the target renderer or not. For example, a texture, a fractal,
/// or a mix node.
    pub unsafe struct Shader as "pxr::UsdShadeShader"
);

impl Shader {
    pub fn new(prim: &Prim) -> Shader {
        unsafe {
            cpp!([prim as "pxr::UsdPrim*"]
                        -> Shader as "pxr::UsdShadeShader" {
                return pxr::UsdShadeShader::Get(prim->GetStage(), prim->GetPath());
            })
        }
    }

    /// Return this schema object's held prim.
    pub fn get_prim(&self) -> Prim {
        unsafe {
            cpp!([self as "pxr::UsdShadeShader*"]
                        -> Prim as "pxr::UsdPrim" {
                return self->GetPrim();
            })
        }
    }
}
