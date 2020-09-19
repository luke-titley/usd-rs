//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

// Supported types.
// Taken from :
//      http://graphics.pixar.com/usd/docs/api/_usd__page__datatypes.html
//
//! bool		bool
//! uchar		uint8_t	8 bit unsigned integer
//! int		    int32_t	32 bit signed integer
//! uint		uint32_t	32 bit unsigned integer
//! int64		int64_t	64 bit signed integer
//! uint64		uint64_t	64 bit unsigned integer
//! half		GfHalf	16 bit floating point
//! float		float	32 bit floating point
//! double		double	64 bit floating point
//! timecode	SdfTimeCode	double representing a resolvable time
//! string		std::string	stl string
//! token		TfToken	interned string with fast comparison and hashing
//! asset		SdfAssetPath	represents a resolvable path to another asset
//! matrix2d	GfMatrix2d	2x2 matrix of doubles
//! matrix3d	GfMatrix3d	3x3 matrix of doubles
//! matrix4d	GfMatrix4d	4x4 matrix of doubles
//! quatd		GfQuatd	double-precision quaternion
//! quatf		GfQuatf	single-precision quaternion
//! quath		GfQuath	half-precision quaternion
//! double2		GfVec2d	vector of 2 doubles
//! float2		GfVec2f	vector of 2 floats
//! half2		GfVec2h	vector of 2 half's
//! int2		GfVec2i	vector of 2 ints
//! double3		GfVec3d	vector of 3 doubles
//! float3		GfVec3f	vector of 3 floats
//! half3		GfVec3h	vector of 3 half's
//! int3		GfVec3i	vector of 3 ints
//! double4		GfVec4d	vector of 4 doubles
//! float4		GfVec4f	vector of 4 floats
//! half4		GfVec4h	vector of 4 half's
//! int4		GfVec4i	vector of 4 ints

use crate::pxr::tf;
use half::f16;

// TODO: Support arrays
pub enum BasicValue {
    Bool(bool),
    UChar(u8),
    Int(i32),
    UInt(u32),
    Int64(i64),
    UInt64(u64),
    Half(f16),
    Float(f32),
    Double(f64),
    // TimeCode(sdf::TimeCode), // TODO: LT
    String(std::string::String),
    Token(tf::Token),
    // Asset(sdf::AssetPath), // TODO: LT
    Matrix2d([f64; 2 * 2]),
    Matrix3d([f64; 3 * 3]),
    Matrix4d([f64; 4 * 4]),
    Quatd([f64; 4]),
    Quatf([f32; 4]),
    Quath([f16; 4]),
    Double2([f64; 2]),
    Float2([f32; 2]),
    Half2([f16; 2]),
    Int2([i32; 2]),
    Double3([f64; 3]),
    Float3([f32; 3]),
    Half3([f16; 3]),
    Int3([i32; 3]),
    Double4([f64; 4]),
    Float4([f32; 4]),
    Half4([f16; 4]),
    Int4([i32; 4]),
}

//------------------------------------------------------------------------------
//use crate::pxr::sdf;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/base/vt/value.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct Value as "pxr::VtValue");
