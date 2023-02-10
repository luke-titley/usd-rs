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
/// This is a reference to the underlying UsdXformCache
///
#[repr(C, align(8))]
pub struct XformCacheRef {
    // A private member stops users from being able to construct it directly
    _priv: u8,
}

// Handy alias to reduce copy/paste errors
type RefType = XformCacheRef;

//------------------------------------------------------------------------------
impl XformCacheRef {
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

//------------------------------------------------------------------------------
#[repr(C, align(8))]
pub struct XformCache {
    reference: *mut RefType,
}

//------------------------------------------------------------------------------
impl XformCache {
    pub fn new(time: TimeCode) -> XformCache {
        unsafe {
            cpp!([time as "pxr::UsdTimeCode"]
                        -> XformCache as "pxr::UsdGeomXformCache *" {
                return new pxr::UsdGeomXformCache(time);
            })
        }
    }
}

//------------------------------------------------------------------------------
impl Drop for XformCache {
    fn drop(&mut self) {
        let reference = self.reference.clone();
        unsafe {
            cpp!([reference as "const pxr::UsdGeomXformCache*"] {
                delete reference;
            })
        }
    }
}

//------------------------------------------------------------------------------
impl AsRef<RefType> for XformCache {
    fn as_ref(&self) -> &RefType {
        unsafe { &*(self.reference) }
    }
}

impl AsMut<RefType> for XformCache {
    fn as_mut(&mut self) -> &mut RefType {
        unsafe { &mut *self.reference }
    }
}
