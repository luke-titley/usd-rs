use crate::pxr::tf;
use crate::pxr::usd::Attribute;
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
/// This is a reference to the underlying UsdPrimvar
///
#[repr(C, align(8))]
pub struct PrimvarRef {
    // A private member stops users from being able to construct it directly
    _priv: u8,
}

// Handy alias to reduce copy/paste errors
type RefType = PrimvarRef;

//------------------------------------------------------------------------------
impl PrimvarRef {
    /// Return the Primvar's interpolation.
    ///
    /// Interpolation determines how the Primvar interpolates over
    /// a geometric primitive.
    pub fn get_interpolation(&self) -> tf::Token {
        unsafe {
            cpp!([self as "pxr::UsdGeomPrimvar*"]
                        -> tf::Token as "pxr::TfToken" {
                return self->GetInterpolation();
            })
        }
    }
}

//------------------------------------------------------------------------------
/// Schema wrapper for usd.Attribute for authoring and introspecting attributes
/// that are primvars.
///
/// Primvar provides API for authoring and retrieving the
/// additional data required to encode an attribute as a "Primvar",
/// which is a convenient contraction of RenderMan's "Primitive Variable"
/// concept, which is represented in Alembic as
/// "arbitrary geometry parameters" (arbGeomParams).
#[repr(C, align(8))]
pub struct Primvar {
    reference: *mut RefType,
}

//------------------------------------------------------------------------------
impl Primvar {
    pub fn new(attr: &Attribute) -> Primvar {
        unsafe {
            cpp!([attr as "pxr::UsdAttribute*"]
                        -> Primvar as "pxr::UsdGeomPrimvar*" {
                return new pxr::UsdGeomPrimvar(*attr);
            })
        }
    }
}

//------------------------------------------------------------------------------
impl Drop for Primvar {
    fn drop(&mut self) {
        let reference = self.reference.clone();
        unsafe {
            cpp!([reference as "const pxr::UsdGeomPrimvar*"] {
                delete reference;
            })
        }
    }
}

//------------------------------------------------------------------------------
impl AsRef<RefType> for Primvar {
    fn as_ref(&self) -> &RefType {
        unsafe { &*(self.reference) }
    }
}

impl AsMut<RefType> for Primvar {
    fn as_mut(&mut self) -> &mut RefType {
        unsafe { &mut *self.reference }
    }
}

//------------------------------------------------------------------------------
impl std::ops::Deref for Primvar {
    type Target = RefType;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

//------------------------------------------------------------------------------
impl std::ops::DerefMut for Primvar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
