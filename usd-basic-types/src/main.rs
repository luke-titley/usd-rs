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
const BASIC_TYPES: [(&str, &str, Option<&str>); 9] = [
    ("bool", "bool", None),
    ("u8", "uint8_t", None),
    ("i32", "int32_t", None),
    ("u32", "uint32_t", None),
    ("i64", "int64_t", None),
    ("u64", "uint64_t", None),
    ("f16", "pxr::GfHalf", Some("pxr/base/gf/half.h")),
    ("f32", "float", None),
    ("f64", "double", None),
];

/// Generate the code needed to get/set the basic types, using a vt::Value.
/// This is the code path that is used for getting the setting the values of
/// an attribute.
fn generate_basic_types() {
    let headers: std::string::String = BASIC_TYPES
        .iter()
        .filter(|(_, _, x)| x.is_some())
        .map(|(_, _, x)| format!(r#"#include "{}"\n"#, x.unwrap()))
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

cpp! {{{{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/base/vt/value.h"
    {headers}
    #pragma GCC diagnostic pop
}}}}
"#,
        headers = &headers
    );

    for (r, c, _) in BASIC_TYPES.iter() {
        println!(
            r#"impl From<&{r}> for Value {{
    fn from(other: &{r}) -> Self {{
        unsafe {{
            cpp!([other as "const {c} *"] -> Value as "pxr::VtValue" {{
                return pxr::VtValue(*other);
            }})
        }}
    }}
}}

impl AsRef<{r}> for Value {{
    fn as_ref(&self) -> &{r} {{
        unsafe {{
            cpp!([self as "const pxr::VtValue *"] -> &{r} as "const {c} *" {{
                return &(self->Get<{c}>());
            }})
        }}
    }}
}}"#,
            r = r,
            c = c
        );
    }
}

fn main() {
    generate_basic_types();
}
