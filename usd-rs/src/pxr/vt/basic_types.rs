//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
// AUTOMATICALLY GENERATED : Don't edit by hand.
// See usd-basic-types

use super::Value;
use cpp::*;

use half::f16; // Half is not a standard rust type

// To avoid a conflict between types, like vec4 and quat, we use named tuples.
pub struct Bool(pub bool);
pub struct UChar(pub u8);
pub struct Int(pub i32);
pub struct UInt(pub u32);
pub struct Int64(pub i64);
pub struct UInt64(pub u64);
pub struct Half(pub f16);
pub struct Float(pub f32);
pub struct Double(pub f64);
pub struct Token(pub crate::pxr::tf::Token);
pub struct Matrix2d(pub [f64;2*3]);
pub struct Matrix3d(pub [f64;3*3]);
pub struct Matrix4d(pub [f64;4*4]);
pub struct Quatd(pub [f64;4]);
pub struct Quatf(pub [f32;4]);
pub struct Quath(pub [f16;4]);
pub struct Vec2d(pub [f64;2]);
pub struct Vec2f(pub [f32;2]);
pub struct Vec2h(pub [f16;2]);
pub struct Vec2i(pub [i32;2]);
pub struct Vec3d(pub [f64;3]);
pub struct Vec3f(pub [f32;3]);
pub struct Vec3h(pub [f16;3]);
pub struct Vec3i(pub [i32;3]);
pub struct Vec4d(pub [f64;4]);
pub struct Vec4f(pub [f32;4]);
pub struct Vec4h(pub [f16;4]);
pub struct Vec4i(pub [i32;4]);


cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/base/vt/value.h"
    #include "pxr/base/gf/half.h"
    #include "pxr/base/tf/token.h"
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
