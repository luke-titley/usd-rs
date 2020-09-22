//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
// AUTOMATICALLY GENERATED : Don't edit by hand.
// See usd-basic-types

use super::Value;
use cpp::*;

use half::f16; // Half is not a standard rust type

// To avoid a conflict between types, like vec4 and quat, we use tuple structs.
// repr(transparent) ensures that the struct is exactly the same size in memory
// as the type it is wrapping. This allows us to safely cast forwards and
// backwards between the types.
//
// We provide a 'From<&T>' implementation for the tuple structs to
// provide a means to do a zero copy transfer from rust over to c++.
#[repr(transparent)]
pub struct Bool(pub bool);

impl From<&bool> for &Bool {
    fn from(other : &bool) -> Self {
        unsafe { &*((other as *const bool) as *const Bool) }
    }
}
#[repr(transparent)]
pub struct UChar(pub u8);

impl From<&u8> for &UChar {
    fn from(other : &u8) -> Self {
        unsafe { &*((other as *const u8) as *const UChar) }
    }
}
#[repr(transparent)]
pub struct Int(pub i32);

impl From<&i32> for &Int {
    fn from(other : &i32) -> Self {
        unsafe { &*((other as *const i32) as *const Int) }
    }
}
#[repr(transparent)]
pub struct UInt(pub u32);

impl From<&u32> for &UInt {
    fn from(other : &u32) -> Self {
        unsafe { &*((other as *const u32) as *const UInt) }
    }
}
#[repr(transparent)]
pub struct Int64(pub i64);

impl From<&i64> for &Int64 {
    fn from(other : &i64) -> Self {
        unsafe { &*((other as *const i64) as *const Int64) }
    }
}
#[repr(transparent)]
pub struct UInt64(pub u64);

impl From<&u64> for &UInt64 {
    fn from(other : &u64) -> Self {
        unsafe { &*((other as *const u64) as *const UInt64) }
    }
}
#[repr(transparent)]
pub struct Half(pub f16);

impl From<&f16> for &Half {
    fn from(other : &f16) -> Self {
        unsafe { &*((other as *const f16) as *const Half) }
    }
}
#[repr(transparent)]
pub struct Float(pub f32);

impl From<&f32> for &Float {
    fn from(other : &f32) -> Self {
        unsafe { &*((other as *const f32) as *const Float) }
    }
}
#[repr(transparent)]
pub struct Double(pub f64);

impl From<&f64> for &Double {
    fn from(other : &f64) -> Self {
        unsafe { &*((other as *const f64) as *const Double) }
    }
}
#[repr(transparent)]
pub struct TimeCode(pub crate::pxr::sdf::TimeCode);

impl From<&crate::pxr::sdf::TimeCode> for &TimeCode {
    fn from(other : &crate::pxr::sdf::TimeCode) -> Self {
        unsafe { &*((other as *const crate::pxr::sdf::TimeCode) as *const TimeCode) }
    }
}
#[repr(transparent)]
pub struct Token(pub crate::pxr::tf::Token);

impl From<&crate::pxr::tf::Token> for &Token {
    fn from(other : &crate::pxr::tf::Token) -> Self {
        unsafe { &*((other as *const crate::pxr::tf::Token) as *const Token) }
    }
}
#[repr(transparent)]
pub struct Asset(pub crate::pxr::sdf::AsstPth);

impl From<&crate::pxr::sdf::AsstPth> for &Asset {
    fn from(other : &crate::pxr::sdf::AsstPth) -> Self {
        unsafe { &*((other as *const crate::pxr::sdf::AsstPth) as *const Asset) }
    }
}
#[repr(transparent)]
pub struct Matrix2d(pub [f64;2*3]);

impl From<&[f64;2*3]> for &Matrix2d {
    fn from(other : &[f64;2*3]) -> Self {
        unsafe { &*((other as *const [f64;2*3]) as *const Matrix2d) }
    }
}
#[repr(transparent)]
pub struct Matrix3d(pub [f64;3*3]);

impl From<&[f64;3*3]> for &Matrix3d {
    fn from(other : &[f64;3*3]) -> Self {
        unsafe { &*((other as *const [f64;3*3]) as *const Matrix3d) }
    }
}
#[repr(transparent)]
pub struct Matrix4d(pub [f64;4*4]);

impl From<&[f64;4*4]> for &Matrix4d {
    fn from(other : &[f64;4*4]) -> Self {
        unsafe { &*((other as *const [f64;4*4]) as *const Matrix4d) }
    }
}
#[repr(transparent)]
pub struct Quatd(pub [f64;4]);

impl From<&[f64;4]> for &Quatd {
    fn from(other : &[f64;4]) -> Self {
        unsafe { &*((other as *const [f64;4]) as *const Quatd) }
    }
}
#[repr(transparent)]
pub struct Quatf(pub [f32;4]);

impl From<&[f32;4]> for &Quatf {
    fn from(other : &[f32;4]) -> Self {
        unsafe { &*((other as *const [f32;4]) as *const Quatf) }
    }
}
#[repr(transparent)]
pub struct Quath(pub [f16;4]);

impl From<&[f16;4]> for &Quath {
    fn from(other : &[f16;4]) -> Self {
        unsafe { &*((other as *const [f16;4]) as *const Quath) }
    }
}
#[repr(transparent)]
pub struct Vec2d(pub [f64;2]);

impl From<&[f64;2]> for &Vec2d {
    fn from(other : &[f64;2]) -> Self {
        unsafe { &*((other as *const [f64;2]) as *const Vec2d) }
    }
}
#[repr(transparent)]
pub struct Vec2f(pub [f32;2]);

impl From<&[f32;2]> for &Vec2f {
    fn from(other : &[f32;2]) -> Self {
        unsafe { &*((other as *const [f32;2]) as *const Vec2f) }
    }
}
#[repr(transparent)]
pub struct Vec2h(pub [f16;2]);

impl From<&[f16;2]> for &Vec2h {
    fn from(other : &[f16;2]) -> Self {
        unsafe { &*((other as *const [f16;2]) as *const Vec2h) }
    }
}
#[repr(transparent)]
pub struct Vec2i(pub [i32;2]);

impl From<&[i32;2]> for &Vec2i {
    fn from(other : &[i32;2]) -> Self {
        unsafe { &*((other as *const [i32;2]) as *const Vec2i) }
    }
}
#[repr(transparent)]
pub struct Vec3d(pub [f64;3]);

impl From<&[f64;3]> for &Vec3d {
    fn from(other : &[f64;3]) -> Self {
        unsafe { &*((other as *const [f64;3]) as *const Vec3d) }
    }
}
#[repr(transparent)]
pub struct Vec3f(pub [f32;3]);

impl From<&[f32;3]> for &Vec3f {
    fn from(other : &[f32;3]) -> Self {
        unsafe { &*((other as *const [f32;3]) as *const Vec3f) }
    }
}
#[repr(transparent)]
pub struct Vec3h(pub [f16;3]);

impl From<&[f16;3]> for &Vec3h {
    fn from(other : &[f16;3]) -> Self {
        unsafe { &*((other as *const [f16;3]) as *const Vec3h) }
    }
}
#[repr(transparent)]
pub struct Vec3i(pub [i32;3]);

impl From<&[i32;3]> for &Vec3i {
    fn from(other : &[i32;3]) -> Self {
        unsafe { &*((other as *const [i32;3]) as *const Vec3i) }
    }
}
#[repr(transparent)]
pub struct Vec4d(pub [f64;4]);

impl From<&[f64;4]> for &Vec4d {
    fn from(other : &[f64;4]) -> Self {
        unsafe { &*((other as *const [f64;4]) as *const Vec4d) }
    }
}
#[repr(transparent)]
pub struct Vec4f(pub [f32;4]);

impl From<&[f32;4]> for &Vec4f {
    fn from(other : &[f32;4]) -> Self {
        unsafe { &*((other as *const [f32;4]) as *const Vec4f) }
    }
}
#[repr(transparent)]
pub struct Vec4h(pub [f16;4]);

impl From<&[f16;4]> for &Vec4h {
    fn from(other : &[f16;4]) -> Self {
        unsafe { &*((other as *const [f16;4]) as *const Vec4h) }
    }
}
#[repr(transparent)]
pub struct Vec4i(pub [i32;4]);

impl From<&[i32;4]> for &Vec4i {
    fn from(other : &[i32;4]) -> Self {
        unsafe { &*((other as *const [i32;4]) as *const Vec4i) }
    }
}


cpp! {{
    #include <string>

    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/base/vt/value.h"
    #include "pxr/base/gf/half.h"
    #include "pxr/usd/sdf/timeCode.h"
    #include "pxr/base/tf/token.h"
    #include "pxr/usd/sdf/assetPath.h"
    #include "pxr/base/gf/matrix2d.h"
    #include "pxr/base/gf/matrix3d.h"
    #include "pxr/base/gf/matrix4d.h"
    #include "pxr/base/gf/quatd.h"
    #include "pxr/base/gf/quatf.h"
    #include "pxr/base/gf/quath.h"
    #include "pxr/base/gf/vec2d.h"
    #include "pxr/base/gf/vec2f.h"
    #include "pxr/base/gf/vec2h.h"
    #include "pxr/base/gf/vec2i.h"
    #include "pxr/base/gf/vec3d.h"
    #include "pxr/base/gf/vec3f.h"
    #include "pxr/base/gf/vec3h.h"
    #include "pxr/base/gf/vec3i.h"
    #include "pxr/base/gf/vec4d.h"
    #include "pxr/base/gf/vec4f.h"
    #include "pxr/base/gf/vec4h.h"
    #include "pxr/base/gf/vec4i.h"

    #pragma GCC diagnostic pop
}}

// The String type is written by hand because:
// - The constructor for VtValue takes a pointer to "const char *", but internally
//  stores a std::string.
// - CStr::as_ptr has to be called to convert to a "const char *".
// - CStr::from has to be called to convert from a "const char *".
//
// Why did we choose CStr to get/set string vt::Values ?
//
// The options available are:
//  - &std::string::str
//  - std::string::String
//  - std::ffi::CString,
//  - std::ffi::CStr
//  - * const std::os::raw::c_char;
//
// The standard rust str and String types are for unicode strings, moving to and
// from them requires a test for valid unicode. This goes against the low
// overhead goal of this binding.
//
// Converting to and from * const std::os::raw::c_char has basically no cost.
// But you need an unsafe block to do anything useful with pointers, and this
// goes against the primary goal of this binding, which is to be a safe
// api.
//
// CString/CStr provides a string type that matches c strings, complete with a
// null terminator. So we can ensure converting them to 'const char *' will have
// very little cost. CString is the owned representation (like std::string), while
// CStr is the reference representation.
//
// There is a cost to using CStr. That is, the API user has to do the conversion
// from &str to &CStr, but by pushing these upwards, there is more opportunity
// to reduce the number of times the conversions need to be done.

#[repr(transparent)]
pub struct String(pub std::ffi::CStr);

impl From<&std::ffi::CStr> for &String {
    fn from(other : &std::ffi::CStr) -> Self {
        unsafe { &*((other as *const std::ffi::CStr) as *const String) }
    }
}

impl From<&String> for Value {
    fn from(other: &String) -> Self {
        let c_char = other.0.as_ptr();
        unsafe {
            cpp!([c_char as "const char *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(c_char);
            })
        }
    }
}

impl AsRef<String> for Value {
    fn as_ref(&self) -> &String {
        use std::os::raw::c_char;

        <&String>::from(
            unsafe {
                std::ffi::CStr::from_ptr(
                
                    cpp!([self as "const pxr::VtValue *"] ->  * const c_char as "const char *" {
                        return self->Get<std::string>().c_str();
                    })

                )
            }
        )
    }
}


impl From<&Bool> for Value {
    fn from(other: &Bool) -> Self {
        unsafe {
            cpp!([other as "const bool *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Bool> for Value {
    fn as_ref(&self) -> &Bool {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Bool as "const bool *" {
                return &(self->Get<bool>());
            })
        }
    }
}
impl From<&UChar> for Value {
    fn from(other: &UChar) -> Self {
        unsafe {
            cpp!([other as "const uint8_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<UChar> for Value {
    fn as_ref(&self) -> &UChar {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &UChar as "const uint8_t *" {
                return &(self->Get<uint8_t>());
            })
        }
    }
}
impl From<&Int> for Value {
    fn from(other: &Int) -> Self {
        unsafe {
            cpp!([other as "const int32_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Int> for Value {
    fn as_ref(&self) -> &Int {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Int as "const int32_t *" {
                return &(self->Get<int32_t>());
            })
        }
    }
}
impl From<&UInt> for Value {
    fn from(other: &UInt) -> Self {
        unsafe {
            cpp!([other as "const uint32_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<UInt> for Value {
    fn as_ref(&self) -> &UInt {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &UInt as "const uint32_t *" {
                return &(self->Get<uint32_t>());
            })
        }
    }
}
impl From<&Int64> for Value {
    fn from(other: &Int64) -> Self {
        unsafe {
            cpp!([other as "const int64_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Int64> for Value {
    fn as_ref(&self) -> &Int64 {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Int64 as "const int64_t *" {
                return &(self->Get<int64_t>());
            })
        }
    }
}
impl From<&UInt64> for Value {
    fn from(other: &UInt64) -> Self {
        unsafe {
            cpp!([other as "const uint64_t *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<UInt64> for Value {
    fn as_ref(&self) -> &UInt64 {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &UInt64 as "const uint64_t *" {
                return &(self->Get<uint64_t>());
            })
        }
    }
}
impl From<&Half> for Value {
    fn from(other: &Half) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfHalf *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Half> for Value {
    fn as_ref(&self) -> &Half {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Half as "const pxr::GfHalf *" {
                return &(self->Get<pxr::GfHalf>());
            })
        }
    }
}
impl From<&Float> for Value {
    fn from(other: &Float) -> Self {
        unsafe {
            cpp!([other as "const float *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Float> for Value {
    fn as_ref(&self) -> &Float {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Float as "const float *" {
                return &(self->Get<float>());
            })
        }
    }
}
impl From<&Double> for Value {
    fn from(other: &Double) -> Self {
        unsafe {
            cpp!([other as "const double *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Double> for Value {
    fn as_ref(&self) -> &Double {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Double as "const double *" {
                return &(self->Get<double>());
            })
        }
    }
}
impl From<&TimeCode> for Value {
    fn from(other: &TimeCode) -> Self {
        unsafe {
            cpp!([other as "const pxr::SdfTimeCode *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<TimeCode> for Value {
    fn as_ref(&self) -> &TimeCode {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &TimeCode as "const pxr::SdfTimeCode *" {
                return &(self->Get<pxr::SdfTimeCode>());
            })
        }
    }
}
impl From<&Token> for Value {
    fn from(other: &Token) -> Self {
        unsafe {
            cpp!([other as "const pxr::TfToken *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Token> for Value {
    fn as_ref(&self) -> &Token {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Token as "const pxr::TfToken *" {
                return &(self->Get<pxr::TfToken>());
            })
        }
    }
}
impl From<&Asset> for Value {
    fn from(other: &Asset) -> Self {
        unsafe {
            cpp!([other as "const pxr::SdfAssetPath *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Asset> for Value {
    fn as_ref(&self) -> &Asset {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Asset as "const pxr::SdfAssetPath *" {
                return &(self->Get<pxr::SdfAssetPath>());
            })
        }
    }
}
impl From<&Matrix2d> for Value {
    fn from(other: &Matrix2d) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfMatrix2d *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Matrix2d> for Value {
    fn as_ref(&self) -> &Matrix2d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Matrix2d as "const pxr::GfMatrix2d *" {
                return &(self->Get<pxr::GfMatrix2d>());
            })
        }
    }
}
impl From<&Matrix3d> for Value {
    fn from(other: &Matrix3d) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfMatrix3d *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Matrix3d> for Value {
    fn as_ref(&self) -> &Matrix3d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Matrix3d as "const pxr::GfMatrix3d *" {
                return &(self->Get<pxr::GfMatrix3d>());
            })
        }
    }
}
impl From<&Matrix4d> for Value {
    fn from(other: &Matrix4d) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfMatrix4d *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Matrix4d> for Value {
    fn as_ref(&self) -> &Matrix4d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Matrix4d as "const pxr::GfMatrix4d *" {
                return &(self->Get<pxr::GfMatrix4d>());
            })
        }
    }
}
impl From<&Quatd> for Value {
    fn from(other: &Quatd) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfQuatd *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Quatd> for Value {
    fn as_ref(&self) -> &Quatd {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Quatd as "const pxr::GfQuatd *" {
                return &(self->Get<pxr::GfQuatd>());
            })
        }
    }
}
impl From<&Quatf> for Value {
    fn from(other: &Quatf) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfQuatf *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Quatf> for Value {
    fn as_ref(&self) -> &Quatf {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Quatf as "const pxr::GfQuatf *" {
                return &(self->Get<pxr::GfQuatf>());
            })
        }
    }
}
impl From<&Quath> for Value {
    fn from(other: &Quath) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfQuath *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Quath> for Value {
    fn as_ref(&self) -> &Quath {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Quath as "const pxr::GfQuath *" {
                return &(self->Get<pxr::GfQuath>());
            })
        }
    }
}
impl From<&Vec2d> for Value {
    fn from(other: &Vec2d) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec2d *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec2d> for Value {
    fn as_ref(&self) -> &Vec2d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec2d as "const pxr::GfVec2d *" {
                return &(self->Get<pxr::GfVec2d>());
            })
        }
    }
}
impl From<&Vec2f> for Value {
    fn from(other: &Vec2f) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec2f *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec2f> for Value {
    fn as_ref(&self) -> &Vec2f {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec2f as "const pxr::GfVec2f *" {
                return &(self->Get<pxr::GfVec2f>());
            })
        }
    }
}
impl From<&Vec2h> for Value {
    fn from(other: &Vec2h) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec2h *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec2h> for Value {
    fn as_ref(&self) -> &Vec2h {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec2h as "const pxr::GfVec2h *" {
                return &(self->Get<pxr::GfVec2h>());
            })
        }
    }
}
impl From<&Vec2i> for Value {
    fn from(other: &Vec2i) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec2i *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec2i> for Value {
    fn as_ref(&self) -> &Vec2i {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec2i as "const pxr::GfVec2i *" {
                return &(self->Get<pxr::GfVec2i>());
            })
        }
    }
}
impl From<&Vec3d> for Value {
    fn from(other: &Vec3d) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec3d *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec3d> for Value {
    fn as_ref(&self) -> &Vec3d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec3d as "const pxr::GfVec3d *" {
                return &(self->Get<pxr::GfVec3d>());
            })
        }
    }
}
impl From<&Vec3f> for Value {
    fn from(other: &Vec3f) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec3f *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec3f> for Value {
    fn as_ref(&self) -> &Vec3f {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec3f as "const pxr::GfVec3f *" {
                return &(self->Get<pxr::GfVec3f>());
            })
        }
    }
}
impl From<&Vec3h> for Value {
    fn from(other: &Vec3h) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec3h *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec3h> for Value {
    fn as_ref(&self) -> &Vec3h {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec3h as "const pxr::GfVec3h *" {
                return &(self->Get<pxr::GfVec3h>());
            })
        }
    }
}
impl From<&Vec3i> for Value {
    fn from(other: &Vec3i) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec3i *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec3i> for Value {
    fn as_ref(&self) -> &Vec3i {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec3i as "const pxr::GfVec3i *" {
                return &(self->Get<pxr::GfVec3i>());
            })
        }
    }
}
impl From<&Vec4d> for Value {
    fn from(other: &Vec4d) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec4d *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec4d> for Value {
    fn as_ref(&self) -> &Vec4d {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec4d as "const pxr::GfVec4d *" {
                return &(self->Get<pxr::GfVec4d>());
            })
        }
    }
}
impl From<&Vec4f> for Value {
    fn from(other: &Vec4f) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec4f *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec4f> for Value {
    fn as_ref(&self) -> &Vec4f {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec4f as "const pxr::GfVec4f *" {
                return &(self->Get<pxr::GfVec4f>());
            })
        }
    }
}
impl From<&Vec4h> for Value {
    fn from(other: &Vec4h) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec4h *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec4h> for Value {
    fn as_ref(&self) -> &Vec4h {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec4h as "const pxr::GfVec4h *" {
                return &(self->Get<pxr::GfVec4h>());
            })
        }
    }
}
impl From<&Vec4i> for Value {
    fn from(other: &Vec4i) -> Self {
        unsafe {
            cpp!([other as "const pxr::GfVec4i *"] -> Value as "pxr::VtValue" {
                return pxr::VtValue(*other);
            })
        }
    }
}

impl AsRef<Vec4i> for Value {
    fn as_ref(&self) -> &Vec4i {
        unsafe {
            cpp!([self as "const pxr::VtValue *"] -> &Vec4i as "const pxr::GfVec4i *" {
                return &(self->Get<pxr::GfVec4i>());
            })
        }
    }
}
