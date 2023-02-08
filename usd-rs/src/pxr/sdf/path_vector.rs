//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use super::path::Path;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/sdf/path.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
/// This is a reference to the underlying SdfPathVector
///
#[repr(C, align(8))]
pub struct PathVectorRef {
    // A private member stops users from being able to construct it directly
    _priv: u8,
}

// Handy alias to reduce copy/paste errors
type RefType = PathVectorRef;

//------------------------------------------------------------------------------
impl PathVectorRef {
    pub fn push(&mut self, path: &Path) {
        unsafe {
            cpp!([self as "pxr::SdfPathVector*", path as "const pxr::SdfPath*"] {
                self->push_back(*path);
            })
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::SdfPathVector*"] -> usize as "size_t" {
                return self->size();
            })
        }
    }
}

impl std::ops::Index<usize> for PathVectorRef {
    type Output = Path;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            cpp!([self as "const pxr::SdfPathVector*", index as "size_t"]
                -> &Path as "const pxr::SdfPath*"
            {
                return &self->at(index);
            })
        }
    }
}

//------------------------------------------------------------------------------
#[repr(C, align(8))]
pub struct PathVector {
    reference: *mut RefType,
}

//------------------------------------------------------------------------------
impl PathVector {
    pub fn new() -> Self {
        unsafe {
            cpp!([] -> PathVector as "const pxr::SdfPathVector*" {
                return new pxr::SdfPathVector();
            })
        }
    }
}

//------------------------------------------------------------------------------
impl Drop for PathVector {
    fn drop(&mut self) {
        let reference = self.reference.clone();
        unsafe {
            cpp!([reference as "const pxr::SdfPathVector*"] {
                delete reference;
            })
        }
    }
}

//------------------------------------------------------------------------------
impl AsRef<RefType> for PathVector {
    fn as_ref(&self) -> &RefType {
        unsafe { &*(self.reference) }
    }
}

impl AsMut<RefType> for PathVector {
    fn as_mut(&mut self) -> &mut RefType {
        unsafe { &mut *self.reference }
    }
}
