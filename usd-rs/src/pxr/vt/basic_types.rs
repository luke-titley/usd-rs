//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
// AUTOMATICALLY GENERATED : Don't edit by hand.
// See usd-basic-types

use super::Value;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/base/vt/value.h"
    #pragma GCC diagnostic pop
}}

impl From<&bool> for Value {
    fn from(other: &bool) -> Self {
        unsafe {
            cpp!([other as "const bool *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<bool> for Value {
    fn as_ref(&self) -> &bool {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &bool as "const bool *" {
                return &(self->Get<bool>());
            })
        }
    }
}
impl From<&u8> for Value {
    fn from(other: &u8) -> Self {
        unsafe {
            cpp!([other as "const uint8_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<u8> for Value {
    fn as_ref(&self) -> &u8 {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &u8 as "const uint8_t *" {
                return &(self->Get<uint8_t>());
            })
        }
    }
}
impl From<&i32> for Value {
    fn from(other: &i32) -> Self {
        unsafe {
            cpp!([other as "const int32_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<i32> for Value {
    fn as_ref(&self) -> &i32 {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &i32 as "const int32_t *" {
                return &(self->Get<int32_t>());
            })
        }
    }
}
impl From<&u32> for Value {
    fn from(other: &u32) -> Self {
        unsafe {
            cpp!([other as "const uint32_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<u32> for Value {
    fn as_ref(&self) -> &u32 {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &u32 as "const uint32_t *" {
                return &(self->Get<uint32_t>());
            })
        }
    }
}
impl From<&i64> for Value {
    fn from(other: &i64) -> Self {
        unsafe {
            cpp!([other as "const int64_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<i64> for Value {
    fn as_ref(&self) -> &i64 {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &i64 as "const int64_t *" {
                return &(self->Get<int64_t>());
            })
        }
    }
}
impl From<&u64> for Value {
    fn from(other: &u64) -> Self {
        unsafe {
            cpp!([other as "const uint64_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<u64> for Value {
    fn as_ref(&self) -> &u64 {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &u64 as "const uint64_t *" {
                return &(self->Get<uint64_t>());
            })
        }
    }
}
