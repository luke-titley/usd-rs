use crate::pxr::usd::Attribute;
use crate::pxr::usd::Prim;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usdGeom/mesh.h"
    #include "pxr/usd/usdGeom/xformCache.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct Mesh as "pxr::UsdGeomMesh");

impl Mesh {
    pub fn new(prim: &Prim) -> Mesh {
        unsafe {
            cpp!([prim as "pxr::UsdPrim*"]
                        -> Mesh as "pxr::UsdGeomMesh" {
                return pxr::UsdGeomMesh::Get(prim->GetStage(), prim->GetPath());
            })
        }
    }

    pub fn get_points_attribute(&self) -> Attribute {
        unsafe {
            cpp!([self as "pxr::UsdGeomMesh*"]
                        -> Attribute as "pxr::UsdAttribute" {
                return self->GetPointsAttr();
            })
        }
    }
}
