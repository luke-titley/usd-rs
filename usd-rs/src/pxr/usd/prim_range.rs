//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

use crate::pxr::usd::Prim;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/primRange.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub(crate) unsafe struct PrimRangeIteratorPos as
            "pxr::UsdPrimRange::iterator");

impl PrimRangeIteratorPos {
    pub fn increment(&mut self) {
        unsafe {
            cpp!([self as "pxr::UsdPrimRange::iterator *"] {
                ++(*self);
            });
        }
    }

    pub fn dereference(&self) -> Prim {
        unsafe {
            cpp!([self as "const pxr::UsdPrimRange::iterator *"]
                -> Prim as "pxr::UsdPrim" {
                return *(*self);
            })
        }
    }

    pub fn eq(&self, rhs: &Self) -> bool {
        unsafe {
            cpp!([self as "const pxr::UsdPrimRange::iterator *",
                  rhs as "const pxr::UsdPrimRange::iterator *"]
                -> bool as "bool" {
                return (*self) == (*rhs);
            })
        }
    }
}

pub struct PrimRangeIterator {
    it: PrimRangeIteratorPos,
    end: PrimRangeIteratorPos,
}

impl PrimRangeIterator {
    pub(crate) fn new(begin: PrimRangeIteratorPos, end: PrimRangeIteratorPos) -> Self {
        Self { it: begin, end }
    }
}

impl std::iter::Iterator for PrimRangeIterator {
    type Item = Prim;
    fn next(&mut self) -> Option<Self::Item> {
        if self.it.eq(&self.end) {
            None
        } else {
            let result = self.it.dereference();
            self.it.increment();
            Some(result)
        }
    }
}

#[repr(C, align(8))]
pub(crate) struct PrmRange {}

//------------------------------------------------------------------------------
/// An forward-iterable range that traverses a subtree of prims rooted at a
/// given prim in depth-first order.
///
/// In addition to depth-first order, UsdPrimRange provides the optional ability
/// to traverse in depth-first pre- and post-order wher prims appear twice in
/// the range; first before all descendants and then again immediately after all
/// descendants.  This is useful for maintaining state associated with subtrees,
/// in a stack-like fashion.
#[repr(C, align(8))]
pub struct PrimRange {
    pub(crate) _prim_range: *const PrmRange,
}

impl PrimRange {
    /// Returns an iterator for traversing a [PrimRange].
    /// See [super::Stage::traverse]
    pub fn iter(&self) -> PrimRangeIterator {
        let prim_range = self._prim_range;
        let begin = unsafe {
            cpp!([prim_range as "pxr::UsdPrimRange *"] -> PrimRangeIteratorPos as "pxr::UsdPrimRange::iterator" {
                return prim_range->begin();
            })
        };
        let end = unsafe {
            cpp!([prim_range as "pxr::UsdPrimRange *"] -> PrimRangeIteratorPos as "pxr::UsdPrimRange::iterator" {
                return prim_range->end();
            })
        };

        PrimRangeIterator::new(begin, end)
    }
}

impl Drop for PrimRange {
    fn drop(&mut self) {
        let prim_range = self._prim_range;
        unsafe {
            cpp!([prim_range as "pxr::UsdPrimRange *"] {
                delete prim_range;
            });
            self._prim_range = std::ptr::null_mut();
        };
    }
}
