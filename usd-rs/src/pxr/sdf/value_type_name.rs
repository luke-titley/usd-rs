//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use crate::pxr;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/sdf/valueTypeName.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(
    /// Represents a value type name, i.e. an attribute's type name.  Usually,
    /// a value type name associates a string with a tf.Type and an optional
    /// role, along with additional metadata.  A schema registers all known
    /// value type names and may register multiple names for the same TfType
    /// and role pair.  All name strings for a given pair are collectively
    /// called its aliases.
    ///
    /// A value type name may also represent just a name string, without a
    /// tf.Type, role or other metadata.  This is currently used exclusively
    /// to unserialize and re-serialize an attribute's type name where that
    /// name is not known to the schema.
    ///
    /// Because value type names can have aliases and those aliases may change
    /// in the future, clients should avoid using the value type name's string
    /// representation except to report human readable messages and when
    /// serializing.  Clients can look up a value type name by string using
    /// sdf.SchemaBase::FindType() and shouldn't otherwise need the string.
    /// Aliases compare equal, even if registered by different schemas.
    pub unsafe struct ValueTypeName as "pxr::SdfValueTypeName"
);

impl ValueTypeName {
    /// Returns the type name as a token.  This should not be used for
    /// comparison purposes.
    pub fn get_as_token(&self) -> pxr::tf::Token {
        unsafe {
            cpp!([self as "const pxr::SdfValueTypeName *"]
                        -> pxr::tf::Token as "pxr::TfToken" {
                return self->GetAsToken();
            })
        }
    }
}

impl std::fmt::Display for ValueTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(text) = self.get_as_token().get_text() {
            write!(f, "{}", text)
        } else {
            write!(f, "<invalid value type name>")
        }
    }
}
