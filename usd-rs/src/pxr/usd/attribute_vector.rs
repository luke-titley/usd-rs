//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use super::attribute::Attribute;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/attribute.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
/// This is a reference to the underlying [AttributeVector]
#[repr(C, align(8))]
pub struct AttributeVectorRef {
    // A private member stops users from being able to construct it directly
    _priv: u8,
}

// Handy alias to reduce copy/paste errors
type RefType = AttributeVectorRef;

//------------------------------------------------------------------------------
impl AttributeVectorRef {
    pub fn push(&mut self, value: &Attribute) {
        unsafe {
            cpp!([self as "pxr::UsdAttributeVector*", value as "const pxr::UsdAttribute*"] {
                self->push_back(*value);
            })
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            cpp!([self as "const pxr::UsdAttributeVector*"] -> usize as "size_t" {
                return self->size();
            })
        }
    }
}

impl std::ops::Index<usize> for AttributeVectorRef {
    type Output = Attribute;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            cpp!([self as "const pxr::UsdAttributeVector*", index as "size_t"]
                -> &Attribute as "const pxr::UsdAttribute*"
            {
                return &self->at(index);
            })
        }
    }
}

//------------------------------------------------------------------------------
/// A contiguous block of [Attribute]s
#[repr(C, align(8))]
pub struct AttributeVector {
    reference: *mut RefType,
}

//------------------------------------------------------------------------------
impl AttributeVector {
    pub fn new() -> Self {
        unsafe {
            cpp!([] -> AttributeVector as "const pxr::UsdAttributeVector*" {
                return new pxr::UsdAttributeVector();
            })
        }
    }
}

//------------------------------------------------------------------------------
impl Drop for AttributeVector {
    fn drop(&mut self) {
        let reference = self.reference.clone();
        unsafe {
            cpp!([reference as "const pxr::UsdAttributeVector*"] {
                delete reference;
            })
        }
    }
}

//------------------------------------------------------------------------------
impl AsRef<RefType> for AttributeVector {
    fn as_ref(&self) -> &RefType {
        unsafe { &*(self.reference) }
    }
}

impl AsMut<RefType> for AttributeVector {
    fn as_mut(&mut self) -> &mut RefType {
        unsafe { &mut *self.reference }
    }
}

//------------------------------------------------------------------------------
impl std::ops::Deref for AttributeVector {
    type Target = RefType;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

//------------------------------------------------------------------------------
impl std::ops::DerefMut for AttributeVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
