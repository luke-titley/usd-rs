//------------------------------------------------------------------------------
use crate::pxr;
use crate::pxr::tf;
use crate::pxr::usd::TimeCode;
use crate::pxr::vt;

use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #pragma GCC diagnostic ignored "-Wdeprecated-copy"
    #pragma GCC diagnostic ignored "-Wdeprecated-declarations"
    #include "pxr/usd/usd/attribute.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct Attribute as "pxr::UsdAttribute");

/// Scenegraph object for authoring and retrieving numeric, string, and array
/// valued data, sampled over time.
impl Attribute {
    /// Set the value of this attribute in the current UsdEditTarget to
    /// __value at UsdTimeCode__ time, which defaults to __default__.
    ///
    /// Values are authored without regard to this attribute's variability.
    /// For example, time sample values may be authored on a uniform
    /// attribute. However, the USD_VALIDATE_VARIABILITY TF_DEBUG code
    /// will cause debug information to be output if values that are
    /// inconsistent with this attribute's variability are authored.
    /// See UsdAttribute::GetVariability for more details.
    ///
    /// return false and generate an error if type if value does not match
    /// this attribute's defined scene description type __exactly__,
    /// or if there is no existing definition for the attribute.
    pub fn set(&self, value: &vt::Value, time: TimeCode) {
        unsafe {
            cpp!([self as "const pxr::UsdAttribute *",
                  value as "const pxr::VtValue*",
                  time as "pxr::UsdTimeCode"] {
                self->Set(*value, time);
            })
        }
    }

    /// Perform value resolution to fetch the value of this attribute at the
    /// requested UsdTimeCode `time`, which defaults to __default__.
    ///
    /// If no value is authored at `time` but values are authored at other
    /// times, this function will return an interpolated value based on the
    /// stage's interpolation type.
    /// See Usd_AttributeInterpolation.
    ///
    /// This accessor is designed for high performance data-streaming
    /// applications, allowing one to fetch data into the same container
    /// repeatedly, avoiding memory allocations when possible (VtArray
    /// containers will be resized as necessary to conform to the size of
    /// data being read).
    ///
    /// Values are retrieved without regard to this attribute's variability.
    /// For example, a uniform attribute may retrieve time sample values
    /// if any are authored. However, the USD_VALIDATE_VARIABILITY TF_DEBUG
    /// code will cause debug information to be output if values that are
    /// inconsistent with this attribute's variability are retrieved.
    /// See UsdAttribute::GetVariability for more details.
    ///
    /// Return true if there was a value to be read, it was of the type T
    /// requested, and we read it successfully - false otherwise.
    ///
    /// For more details, see Usd_ValueResolution , and also
    /// Usd_AssetPathValuedAttributes for information on how to
    /// retrieve resolved asset paths from sdf.AssetPath valued attributes.
    pub fn get(&self, value: &mut vt::Value, time: TimeCode) {
        unsafe {
            cpp!([self as "const pxr::UsdAttribute *",
                  value as "pxr::VtValue*",
                  time as "pxr::UsdTimeCode"] {
                self->Get(value, time);
            })
        }
    }

    /// Return the full name of this object, i.e. the last component of its
    /// SdfPath in namespace.
    ///
    /// This is equivalent to, but generally cheaper than,
    /// get_path().get_name_token()
    pub fn get_name(&self) -> pxr::Result<&tf::Token> {
        unsafe {
            let token_ptr = cpp!([self as "const pxr::UsdAttribute *"]
                        -> * const tf::Token as "const pxr::TfToken*" {
                return &self->GetName();
            });
            token_ptr
                .as_ref()
                .ok_or(pxr::Error::UnableToDereferencePointer)
        }
    }

    /// Return the "scene description" value type name for this attribute.
    pub fn get_type_name(&self) -> pxr::sdf::ValueTypeName {
        unsafe {
            cpp!([self as "const pxr::UsdAttribute *"]
                        -> pxr::sdf::ValueTypeName as "pxr::SdfValueTypeName" {
                return self->GetTypeName();
            })
        }
    }

    /// Return true if this attribute has an authored default value, authored
    /// time samples or a fallback value provided by a registered schema. If
    /// the attribute has been Usd_AttributeBlocking "blocked", then
    /// return `true` if and only if it has a fallback value.
    pub fn has_value(&self) -> bool {
        unsafe {
            cpp!([self as "const pxr::UsdAttribute *"]
                        -> bool as "bool" {
                return self->HasValue();
            })
        }
    }

    /// Resolve the requested metadatum named `key` into `value`,
    /// returning true on success.
    ///
    /// Return false if `key` was not resolvable, or if `value's`
    /// type differed from that of the resolved metadatum.
    ///
    /// ## Note
    /// For any composition-related metadata, as enumerated in
    /// GetAllMetadata(), this method will return only the strongest
    /// opinion found, not applying the composition rules used by Pcp
    /// to process the data.  For more processed/composed views of
    /// composition data, please refer to the specific interface classes,
    /// such as UsdReferences, UsdInherits, UsdVariantSets, etc.
    ///
    pub fn get_metadata(&self, key: &tf::Token, value: &mut vt::Value) {
        unsafe {
            cpp!([self as "const pxr::UsdAttribute *",
                key as "pxr::TfToken",
                value as "pxr::VtValue*"] {
                self->GetMetadata(key, value);
            })
        }
    }
}
