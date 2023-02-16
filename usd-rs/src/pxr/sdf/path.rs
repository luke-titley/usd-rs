//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use crate::pxr;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/sdf/path.h"
    #pragma GCC diagnostic pop
}}

cpp_class!(
    /// A path value used to locate objects in layers or scenegraphs.
    ///
    /// # Overview
    ///
    /// [Path] is used in several ways:
    /// - As a storage key for addressing and accessing values held in a Layer
    /// - As a namespace identity for scenegraph objects
    /// - As a way to refer to other scenegraph objects through relative paths
    ///
    /// The paths represented by an [Path] class may be either relative or
    /// absolute.  Relative paths are relative to the prim object that contains them
    /// (that is, if an RelationshipSpec target is relative, it is relative to
    /// the SdfPrimSpec object that owns the RelationshipSpec object).
    ///
    /// Path objects can be readily created from and converted back to strings,
    /// but as Path objects, they have behaviors that make it easy and efficient
    /// to work with them. The Path class provides a full range of methods for
    /// manipulating scene paths by appending a namespace child, appending a
    /// relationship target, getting the parent path,
    /// and so on.  Since the Path class uses a node-based representation
    /// internally, you should use the editing functions rather than converting to
    /// and from strings if possible.
    ///
    /// # Path Syntax
    ///
    /// Like a filesystem path, an Path is conceptually just a sequence of
    /// path components.  Unlike a filesystem path, each component has a type,
    /// and the type is indicated by the syntax.
    ///
    /// Two separators are used between parts of a path. A slash ("/") following an
    /// identifier is used to introduce a namespace child. A period (".") following
    /// an identifier is used to introduce a property.  A property may also have
    /// several non-sequential colons (':') in its name to provide a rudimentary
    /// namespace within properties but may not end or begin with a colon.
    ///
    /// A leading slash in the string representation of an Path object indicates
    /// an absolute path.  Two adjacent periods indicate the parent namespace.
    ///
    /// Brackets ("[" and "]") are used to indicate relationship target paths for
    /// relational attributes.
    ///
    /// The first part in a path is assumed to be a namespace child unless
    /// it is preceded by a period.  That means:
    /// - ``/Foo`` is an absolute path specifying the root prim Foo.
    /// - ``/Foo/Bar`` is an absolute path specifying namespace child Bar
    ///     of root prim Foo.
    /// - ``/Foo/Bar.baz`` is an absolute path specifying property \c baz of
    ///     namespace child Bar of root prim Foo.
    /// - ``Foo`` is a relative path specifying namespace child Foo of
    ///     the current prim.
    /// - ``Foo/Bar`` is a relative path specifying namespace child Bar of
    ///     namespace child Foo of the current prim.
    /// - ``Foo/Bar.baz`` is a relative path specifying property \c baz of
    ///     namespace child Bar of namespace child Foo of the current prim.
    /// - ``.foo`` is a relative path specifying the property \c foo of the
    ///     current prim.
    /// - ``/Foo.bar[/Foo.baz].attrib`` is a relational attribute path. The
    ///     relationship ``/Foo.bar`` has a target ``/Foo.baz``. There is a
    ///     relational attribute `attrib` on that relationship->;target pair.
    ///
    /// # A Note on Thread-Safety
    ///
    /// Path is strongly thread-safe, in the sense that zero additional
    /// synchronization is required between threads creating or using Path
    /// values. Just like tf.Token, Path values are immutable. Internally,
    /// Path uses a global prefix tree to efficiently share representations
    /// of paths, and provide fast equality/hashing operations, but
    /// modifications to this table are internally synchronized. Consequently,
    /// as with tf.Token, for best performance it is important to minimize
    /// the number of values created (since it requires synchronized access to
    /// this table) or copied (since it requires atomic ref-counting operations).
    ///
    pub unsafe struct Path as "pxr::SdfPath"
);

impl Path {
    /// Returns the string representation of this path as a c string.
    ///
    /// This function returns a pointer to a persistent c string.
    ///
    /// This function is recommended only for human-readable or diagnostic
    /// output.  Use the [Path] API to manipulate paths.  It is less
    /// error-prone and has better performance.
    pub fn get_text(&self) -> pxr::Result<&str> {
        let text = unsafe {
            std::ffi::CStr::from_ptr(cpp!([self as "const pxr::SdfPath *"]
                    -> * const std::os::raw::c_char as "const char *" {
                return self->GetText();
            }))
        };

        Ok(text.to_str()?)
    }
}

fn from_c_str(path: &std::ffi::CStr) -> Path {
    let path_str = path.as_ptr() as *const std::os::raw::c_char;

    unsafe {
        cpp!([path_str as "const char *"] -> Path as "pxr::SdfPath" {
            return pxr::SdfPath(std::string(path_str));
        })
    }
}

impl std::convert::TryFrom<&str> for Path {
    type Error = pxr::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let path_cstring = std::ffi::CString::new(value)?;

        Ok(from_c_str(path_cstring.as_c_str()))
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(text) = self.get_text() {
            write!(f, "{}", text)
        } else {
            write!(f, "<invalid path>")
        }
    }
}
