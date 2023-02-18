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
cpp_class!(
/// A Material provides a container into which multiple "render targets"
/// can add data that defines a "shading material" for a renderer.  Typically
/// this consists of one or more UsdRelationship properties that target
/// other prims of type __Shader__ - though a target/client is free to add
/// any data that is suitable.  We __strongly advise__ that all targets
/// adopt the convention that all properties be prefixed with a namespace
/// that identifies the target, e.g. "rel ri:surface = </Shaders/mySurf>".
    pub unsafe struct Material as "pxr::UsdShadeMaterial"
);

impl Material {
    pub fn new(prim: &Prim) -> Material {
        unsafe {
            cpp!([prim as "pxr::UsdPrim*"]
                        -> Material as "pxr::UsdShadeMaterial" {
                return pxr::UsdShadeMaterial::Get(prim->GetStage(), prim->GetPath());
            })
        }
    }

    /// Represents the universal "surface" output terminal of a
    /// material.
    ///
    /// | ||
    /// | -- | -- |
    /// | Declaration | `token outputs:surface` |
    /// | C++ Type | TfToken |
    pub fn get_surface_attribute(&self) -> Attribute {
        unsafe {
            cpp!([self as "pxr::UsdShadeMaterial*"]
                        -> Attribute as "pxr::UsdAttribute" {
                return self->GetSurfaceAttr();
            })
        }
    }

    /// Computes the resolved "surface" output source for the given
    /// __contextVector__. Using the earliest renderContext in the contextVector
    /// that produces a valid Shader object.
    ///
    /// If a "surface" output corresponding to each of the renderContexts
    /// does not exist __or__ is not connected to a valid source, then this
    /// checks the ```universal``` surface output.
    ///
    /// Returns an empty Shader object if there is no valid ```surface```
    /// output source for any of the renderContexts in the __contextVector__.
    /// The python version of this method returns a tuple containing three
    /// elements (the source surface shader, sourceName, sourceType).
    pub fn compute_surface_source(&self) -> Shader {
        unsafe {
            cpp!([self as "pxr::UsdShadeMaterial*"]
                        -> Shader as "pxr::UsdShadeShader" {
                return self->ComputeSurfaceSource();
            })
        }
    }
}
