//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

// Supported types.
// Taken from :
//      http://graphics.pixar.com/usd/docs/api/_usd__page__datatypes.html
//

/// bool		bool
/// uchar		uint8_t	8 bit unsigned integer
/// int		    int32_t	32 bit signed integer
/// uint		uint32_t	32 bit unsigned integer
/// int64		int64_t	64 bit signed integer
/// uint64		uint64_t	64 bit unsigned integer
/// half		GfHalf	16 bit floating point
/// float		float	32 bit floating point
/// double		double	64 bit floating point
/// timecode	SdfTimeCode	double representing a resolvable time
/// string		std::string	stl string
/// token		TfToken	interned string with fast comparison and hashing
/// asset		SdfAssetPath	represents a resolvable path to another asset
/// matrix2d	GfMatrix2d	2x2 matrix of doubles
/// matrix3d	GfMatrix3d	3x3 matrix of doubles
/// matrix4d	GfMatrix4d	4x4 matrix of doubles
/// quatd		GfQuatd	double-precision quaternion
/// quatf		GfQuatf	single-precision quaternion
/// quath		GfQuath	half-precision quaternion
/// double2		GfVec2d	vector of 2 doubles
/// float2		GfVec2f	vector of 2 floats
/// half2		GfVec2h	vector of 2 half's
/// int2		GfVec2i	vector of 2 ints
/// double3		GfVec3d	vector of 3 doubles
/// float3		GfVec3f	vector of 3 floats
/// half3		GfVec3h	vector of 3 half's
/// int3		GfVec3i	vector of 3 ints
/// double4		GfVec4d	vector of 4 doubles
/// float4		GfVec4f	vector of 4 floats
/// half4		GfVec4h	vector of 4 half's
/// int4		GfVec4i	vector of 4 ints

//use half::f16;

/// The basic types supported by USD.
/// At the moment rust-cpp doesnt allow us to embed the cpp! macro inside of
/// other macros. So we have to perform the code generation of the AsRef and
/// From trait implementations as a manual step. This isn't such a big deal
/// as basic types are rarely added or removed.
#[rustfmt::skip]
const BASIC_TYPES: [(&str, &str, &str, Option<&str>); 29] = [
    ("Bool", "bool", "bool", None),
    ("UChar", "u8", "uint8_t", None),
    ("Int", "i32", "int32_t", None),
    ("UInt", "u32", "uint32_t", None),
    ("Int64", "i64", "int64_t", None),
    ("UInt64", "u64", "uint64_t", None),
    ("Half", "f16", "pxr::GfHalf", Some("pxr/base/gf/half.h")),
    ("Float", "f32", "float", None),
    ("Double", "f64", "double", None),
    ("TimeCode", "crate::pxr::sdf::TimeCode", "pxr::SdfTimeCode", Some("pxr/usd/sdf/timeCode.h")),
    // string, std::string // TODO,
    ("Token", "crate::pxr::tf::Token", "pxr::TfToken", Some("pxr/base/tf/token.h")),
    // asset, SdfAssetPath // TODO
    ("Matrix2d", "[f64;2*3]", "pxr::GfMatrix2d", Some("pxr/base/gf/matrix2d.h")),
    ("Matrix3d", "[f64;3*3]", "pxr::GfMatrix3d", Some("pxr/base/gf/matrix3d.h")),
    ("Matrix4d", "[f64;4*4]", "pxr::GfMatrix4d", Some("pxr/base/gf/matrix4d.h")),
    ("Quatd", "[f64;4]", "pxr::GfQuatd", Some("pxr/base/gf/quatd.h")),
    ("Quatf", "[f32;4]","pxr::GfQuatf", Some("pxr/base/gf/quatf.h")),
    ("Quath", "[f16;4]", "pxr::GfQuath", Some("pxr/base/gf/quath.h")),
    ("Vec2d", "[f64;2]", "pxr::GfVec2d", Some("pxr/base/gf/vec2d.h")),
    ("Vec2f", "[f32;2]", "pxr::GfVec2f", Some("pxr/base/gf/vec2f.h")),
    ("Vec2h", "[f16;2]", "pxr::GfVec2h", Some("pxr/base/gf/vec2h.h")),
    ("Vec2i", "[i32;2]", "pxr::GfVec2i", Some("pxr/base/gf/vec2i.h")),
    ("Vec3d", "[f64;3]", "pxr::GfVec3d", Some("pxr/base/gf/vec3d.h")),
    ("Vec3f", "[f32;3]", "pxr::GfVec3f", Some("pxr/base/gf/vec3f.h")),
    ("Vec3h", "[f16;3]", "pxr::GfVec3h", Some("pxr/base/gf/vec3h.h")),
    ("Vec3i", "[i32;3]", "pxr::GfVec3i", Some("pxr/base/gf/vec3i.h")),
    ("Vec4d", "[f64;4]", "pxr::GfVec4d", Some("pxr/base/gf/vec4d.h")),
    ("Vec4f", "[f32;4]", "pxr::GfVec4f", Some("pxr/base/gf/vec4f.h")),
    ("Vec4h", "[f16;4]", "pxr::GfVec4h", Some("pxr/base/gf/vec4h.h")),
    ("Vec4i", "[i32;4]", "pxr::GfVec4i", Some("pxr/base/gf/vec4i.h")),
];

/// Generate the code needed to get/set the basic types, using a vt::Value.
/// This is the code path that is used for getting the setting the values of
/// an attribute.
fn generate_basic_types() {
    let headers: std::string::String = BASIC_TYPES
        .iter()
        .filter(|(_, _, _, x)| x.is_some())
        .map(|(_, _, _, x)| format!("    #include \"{}\"\n", x.unwrap()))
        .collect();

    let names: std::string::String = BASIC_TYPES
        .iter()
        .map(|(name, typ, _, _)| {
            format!(
                "#[repr(transparent)]
pub struct {name}(pub {typ});

impl From<&{typ}> for &{name} {{
    fn from(other : &{typ}) -> Self {{
        unsafe {{ &*((other as *const {typ}) as *const {name}) }}
    }}
}}
",
                name = &name,
                typ = &typ
            )
        })
        .collect();

    println!(
        r#"//------------------------------------------------------------------------------
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
{names}

cpp! {{{{
    #include <string>

    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/base/vt/value.h"
{headers}
    #pragma GCC diagnostic pop
}}}}

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
pub struct String(std::ffi::CStr);

impl From<&std::ffi::CStr> for &String {{
    fn from(other : &std::ffi::CStr) -> Self {{
        unsafe {{ &*((other as *const std::ffi::CStr) as *const String) }}
    }}
}}

impl From<&String> for Value {{
    fn from(other: &String) -> Self {{
        let c_char = other.0.as_ptr();
        unsafe {{
            cpp!([c_char as "const char *"] -> Value as "pxr::VtValue" {{
                return pxr::VtValue(c_char);
            }})
        }}
    }}
}}

impl AsRef<String> for Value {{
    fn as_ref(&self) -> &String {{
        use std::os::raw::c_char;

        <&String>::from(
            unsafe {{
                std::ffi::CStr::from_ptr(
                
                    cpp!([self as "const pxr::VtValue *"] ->  * const c_char as "const char *" {{
                        return self->Get<std::string>().c_str();
                    }})

                )
            }}
        )
    }}
}}

"#,
        headers = &headers,
        names = &names,
    );

    for (name, _, c, _) in BASIC_TYPES.iter() {
        println!(
            r#"impl From<&{name}> for Value {{
    fn from(other: &{name}) -> Self {{
        unsafe {{
            cpp!([other as "const {c} *"] -> Value as "pxr::VtValue" {{
                return pxr::VtValue(*other);
            }})
        }}
    }}
}}

impl AsRef<{name}> for Value {{
    fn as_ref(&self) -> &{name} {{
        unsafe {{
            cpp!([self as "const pxr::VtValue *"] -> &{name} as "const {c} *" {{
                return &(self->Get<{c}>());
            }})
        }}
    }}
}}"#,
            name = name,
            c = c
        );
    }
}

fn main() {
    generate_basic_types();
}
