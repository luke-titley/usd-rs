use crate::pxr::usd::Prim;
use crate::pxr::usd::TimeCode;
use crate::pxr::vt;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usdGeom/xformCache.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct XformCache as "pxr::UsdGeomXformCache"); // TODO LT: This type should be dyanmically allocated like AssetPath.

impl XformCache {
    pub fn new(time: TimeCode) -> XformCache {
        //timecode ?
        unsafe {
            cpp!([time as "pxr::UsdTimeCode"]
                        -> XformCache as "pxr::UsdGeomXformCache" {
                return pxr::UsdGeomXformCache(time);
            })
        }
    }

    pub fn get_local_to_world_transform(&self, prim: &Prim) -> vt::Matrix4d {
        unsafe {
            cpp!([self as "pxr::UsdGeomXformCache*",
                prim as "pxr::UsdPrim*"]
                        -> vt::Matrix4d as "pxr::GfMatrix4d"{
                return self->GetLocalToWorldTransform(*prim);
            })
        }
    }
}
