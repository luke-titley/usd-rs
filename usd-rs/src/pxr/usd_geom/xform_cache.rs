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
    /// Compute the transformation matrix for the given \p prim, including the
    /// transform authored on the Prim itself, if present.
    ///
    /// # Note
    /// This method may mutate internal cache state and is not thread
    /// safe.
    pub fn get_local_to_world_transform(
        &mut self,
        prim: &Prim,
    ) -> vt::Matrix4d {
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
/// A caching mechanism for transform matrices. For best performance, this
/// object should be reused for multiple CTM queries.
///
/// Instances of this type can be copied, though using Swap() may result in
/// better performance.
///
/// It is valid to cache prims from multiple stages in a single XformCache.
///
/// # WARNING
/// This class does not automatically invalidate cached values based
/// on changes to the stage from which values were cached. Additionally, a
/// separate instance of this class should be used per-thread, calling the Get*
/// methods from multiple threads is not safe, as they mutate internal state.
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

//------------------------------------------------------------------------------
impl std::ops::Deref for XformCache {
    type Target = RefType;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

//------------------------------------------------------------------------------
impl std::ops::DerefMut for XformCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
