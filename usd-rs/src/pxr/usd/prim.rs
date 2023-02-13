//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
//use crate::pxr::sdf;
use crate::pxr;
use crate::pxr::sdf;
use crate::pxr::tf;
use crate::pxr::usd::attribute::*;
use crate::pxr::usd::attribute_vector::*;
use crate::pxr::usd::references::References;
use crate::pxr::usd::relationship::Relationship;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/prim.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
pub mod desc {
    use super::*;
    use crate::pxr::sdf;

    pub struct CreateAttribute {
        pub name: tf::Token,
        pub type_name: sdf::ValueTypeName,
        //variability: Option<sdf::Variability>, // TODO
    }
}

//------------------------------------------------------------------------------
/// \class UsdPrim
///
/// UsdPrim is the sole persistent scenegraph object on a UsdStage, and
/// is the embodiment of a "Prim" as described in the <em>Universal Scene
/// Description Composition Compendium</em>
///
/// A UsdPrim is the principal container of other types of scene description.
/// It provides API for accessing and creating all of the contained kinds
/// of scene description, which include:
/// \li UsdVariantSets - all VariantSets on the prim (GetVariantSets(),
/// GetVariantSet()).
/// \li UsdReferences - all references on the prim (GetReferences())
/// \li UsdInherits - all inherits on the prim (GetInherits())
/// \li UsdSpecializes - all specializes on the prim (GetSpecializes())
///
/// As well as access to the API objects for properties contained within the
/// prim - UsdPrim as well as all of the following classes are subclasses
/// of UsdObject:
/// \li UsdProperty - generic access to all attributes and relationships.
/// A UsdProperty can be queried and cast to a UsdAttribute or UsdRelationship
/// using UsdObject::Is<>() and UsdObject::As<>(). (GetPropertyNames(),
/// GetProperties(), GetPropertiesInNamespace(), GetPropertyOrder(),
/// SetPropertyOrder())
/// \li UsdAttribute - access to default and timesampled attribute values, as
/// well as value resolution information, and attribute-specific metadata
/// (CreateAttribute(), GetAttribute(), GetAttributes(), HasAttribute())
/// \li UsdRelationship - access to authoring and resolving relationships
/// to other prims and properties (CreateRelationship(), GetRelationship(),
/// GetRelationships(), HasRelationship())
///
/// UsdPrim also provides access to iteration through its prim children,
/// optionally making use of the \ref primFlags.h "prim predicates facility"
/// (GetChildren(), GetAllChildren(), GetFilteredChildren()).
cpp_class!(pub unsafe struct Prim as "pxr::UsdPrim");


impl Prim {
    /// Return this prim's composed type name. This value is cached and is
    /// efficient to query.
    /// Note that this is just the composed type name as authored and may not
    /// represent the full type of the prim and its prim definition. If you
    /// need to reason about the actual type of the prim, use GetPrimTypeInfo
    /// instead as it accounts for recognized schemas, applied API schemas,
    /// fallback types, etc.
    pub fn get_type_name(&self) -> &tf::Token {
        unsafe {
            cpp!([self as "const pxr::UsdPrim*"]
                        -> * const tf::Token as "const pxr::TfToken*" {
                return &self->GetTypeName();
            })
            .as_ref()
            .unwrap()
        }
    }

    /// Return a UsdReferences object that allows one to add, remove, or
    /// mutate references <em>at the currently set UsdEditTarget</em>.
    ///
    /// While the UsdReferences object has no methods for \em listing the
    /// currently authored references on a prim, one can use a
    /// UsdPrimCompositionQuery to query the reference arcs that are composed
    /// by this prim.
    ///
    /// \sa UsdPrimCompositionQuery::GetDirectReferences
    pub fn get_references(&self) -> References {
        unsafe {
            cpp!([self as "const pxr::UsdPrim*"]
                        -> References as "const pxr::UsdReferences" {
                return self->GetReferences();
            })
        }
    }

    /// Like GetProperties(), but exclude all attributes from the result.
    pub fn get_relationship(&self, rel_name: &tf::Token) -> Relationship {
        unsafe {
            cpp!([self as "const pxr::UsdPrim*",
                rel_name as "pxr::TfToken*"]
                        -> Relationship as "const pxr::UsdRelationship" {
                return self->GetRelationship(*rel_name);
            })
        }
    }

    /// Return true if this prim has an attribute named \p attrName, false
    /// otherwise.
    pub fn has_attribute(&self, attr_name: &tf::Token) -> bool {
        unsafe {
            cpp!([self as "pxr::UsdPrim*",
                attr_name as "pxr::TfToken*"]
                        -> bool as "bool" {
                return self->HasAttribute(*attr_name);
            })
        }
    }

    /// Return true if this prim has a relationship named \p relName, false
    /// otherwise.
    pub fn has_relationship(&self, rel_name: &tf::Token) -> bool {
        unsafe {
            cpp!([self as "const pxr::UsdPrim*",
                rel_name as "pxr::TfToken*"]
                        -> bool as "bool" {
                return self->HasRelationship(*rel_name);
            })
        }
    }

    /// Author scene description for the attribute named \a attrName at the
    /// current EditTarget if none already exists.  Return a valid attribute if
    /// scene description was successfully authored or if it already existed,
    /// return invalid attribute otherwise.  Note that the supplied \a typeName
    /// and \a custom arguments are only used in one specific case.  See below
    /// for details.
    ///
    /// Suggested use:
    /// \code
    /// if (UsdAttribute myAttr = prim.CreateAttribute(...)) {
    ///    // success.
    /// }
    /// \endcode
    ///
    /// To call this, GetPrim() must return a valid prim.
    ///
    /// - If a spec for this attribute already exists at the current edit
    /// target, do nothing.
    ///
    /// - If a spec for \a attrName of a different spec type (e.g. a
    /// relationship) exists at the current EditTarget, issue an error.
    ///
    /// - If \a name refers to a builtin attribute according to the prim's
    /// definition, author an attribute spec with required metadata from the
    /// definition.
    ///
    /// - If \a name refers to a builtin relationship, issue an error.
    ///
    /// - If there exists an absolute strongest authored attribute spec for
    /// \a attrName, author an attribute spec at the current EditTarget by
    /// copying required metadata from that strongest spec.
    ///
    /// - If there exists an absolute strongest authored relationship spec for
    /// \a attrName, issue an error.
    ///
    /// - Otherwise author an attribute spec at the current EditTarget using
    /// the provided \a typeName and \a custom for the required metadata fields.
    /// Note that these supplied arguments are only ever used in this particular
    /// circumstance, in all other cases they are ignored.
    pub fn create_attribute(&self, desc: desc::CreateAttribute) -> Attribute {
        let name = &desc.name;
        let type_name = &desc.type_name;

        unsafe {
            cpp!([self as "pxr::UsdPrim*",
                  name as "pxr::TfToken*",
                  type_name as "pxr::SdfValueTypeName*"]
                        -> Attribute as "pxr::UsdAttribute" {
                return self->CreateAttribute(*name, *type_name);
            })
        }
    }

    pub fn get_attribute(&self, attr_name: &tf::Token) -> Attribute {
        unsafe {
            cpp!([self as "pxr::UsdPrim*",
                attr_name as "pxr::TfToken*"]
                        -> Attribute as "pxr::UsdAttribute" {
                return self->GetAttribute(*attr_name);
            })
        }
    }

    pub fn get_attributes(&self) -> AttributeVector {
        let result = AttributeVector::new(); // This should be 'mut' but compiler incorrectly complains because it cant see the c++
        unsafe {
            cpp!([self as "pxr::UsdPrim*", result as "pxr::UsdAttributeVector*"] {
                *result = self->GetAttributes();
            })
        };
        result
    }

    pub fn get_name(&self) -> pxr::Result<&tf::Token> {
        unsafe {
            let token_ptr = cpp!([self as "const pxr::UsdPrim*"]
                        -> * const tf::Token as "const pxr::TfToken*" {
                return &self->GetName();
            });

            token_ptr
                .as_ref()
                .ok_or(pxr::Error::UnableToDereferencePointer)
        }
    }

    pub fn get_path(&self) -> sdf::Path {
        unsafe {
            cpp!([self as "const pxr::UsdPrim*"]
                        -> sdf::Path as "pxr::SdfPath" {
                return self->GetPath();
            })
        }
    }
}
