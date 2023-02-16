//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

use crate::pxr;

use cpp::*;
use std::ffi::CStr;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/base/tf/token.h"
    #pragma GCC diagnostic pop
}}

cpp_class!(pub unsafe struct Token as "pxr::TfToken");

/// Token for efficient comparison, assignment, and hashing of known strings.
///
/// A TfToken is a handle for a registered string, and can be compared,
/// assigned, and hashed in constant time. It is useful when a bounded number of
/// strings are used as fixed symbols (but never modified).
///
/// For example, the set of avar names in a shot is large but bounded, and once
/// an avar name is discovered, it is never manipulated. If these names were
/// passed around as strings, every comparison and hash would be linear in the
/// number of characters. (String assignment itself is sometimes a constant time
/// operation, but it is sometimes linear in the length of the string as well as
/// requiring a memory allocation.)
///
/// To use TfToken, simply create an instance from a string or const char*. If
/// the string hasn't been seen before, a copy of it is added to a global table.
/// The resulting TfToken is simply a wrapper around an string*, pointing that
/// canonical copy of the string. Thus, operations on the token are very fast.
/// (The string's hash is simply the address of the canonical copy, so hashing
/// the string is constant time.)
///
/// The free functions TfToTokenVector() and TfToStringVector() provide
/// conversions to and from vectors of string.
///
/// Note: Access to the global table is protected by a mutex. This is a good
/// idea as long as clients do not construct tokens from strings too frequently.
/// Construct tokens only as often as you must (for example, as you read data
/// files), and never in inner loops. Of course, once you have a token, feel
/// free to compare, assign, and hash it as often as you like. (That's what it's
/// for.) In order to help prevent tokens from being re-created over and over,
/// auto type conversion from string and char* to TfToken is disabled (you must
/// use the explicit TfToken constructors). However, auto conversion from
/// TfToken to string and char* is provided.
impl Token {
    pub fn get_text(&self) -> pxr::Result<&str> {
        let text = unsafe {
            std::ffi::CStr::from_ptr(cpp!([self as "const pxr::TfToken *"]
                    -> * const std::os::raw::c_char as "const char *" {
                return self->GetText();
            }))
        };

        Ok(text.to_str()?)
    }
}

impl std::cmp::PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            cpp!([self as "const pxr::TfToken *",
                  other as "const pxr::TfToken *"]
                    -> bool as "bool" {
                return (*self) == (*other);
            })
        }
    }
}

fn from_c_str(value: &CStr) -> Token {
    let value_str = value.as_ptr() as *const std::os::raw::c_char;

    unsafe {
        cpp!([value_str as "const char *"] -> Token as "pxr::TfToken" {
            return pxr::TfToken(value_str);
        })
    }
}

impl std::convert::TryFrom<&str> for Token {
    type Error = pxr::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let value_cstring = std::ffi::CString::new(value)?;

        Ok(from_c_str(value_cstring.as_c_str()))
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(text) = self.get_text() {
            write!(f, "{}", text)
        } else {
            write!(f, "<invalid token>")
        }
    }
}
