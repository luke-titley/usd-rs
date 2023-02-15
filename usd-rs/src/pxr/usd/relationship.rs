//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
//use crate::pxr::sdf;
use crate::pxr::sdf;
//use crate::pxr::tf;
//use crate::pxr::usd::attribute::*;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/relationship.h"
    #include <vector>
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct Relationship as "pxr::UsdRelationship");

/// Creates dependencies between scenegraph objects by
/// allowing a prim to `target` other prims, attributes, or relationships.
///
/// # Relationship Characteristics
///
/// A [Relationship] is a pointer to other objects, which are named by their
/// scenegraph paths.  When authoring relationships, the `target` parameters
/// should be scenegraph paths in the composed namespace of the
/// [super::Stage] into which you are authoring.  If your edits are targeted to
/// a different
/// layer, across various composition arcs (because you specified a non-default
/// EditTarget, the target's path will be automatically translated
/// into the proper namespace.
///
/// A single UsdRelationship can target multiple other objects, which can be
/// of [super::Prim], [super::Attribute], or [Relationship] type.  [Relationship]
/// participates in "list editing", which means that stronger layers in a
/// composed scene can add, remove, or reorder targets authored on the
/// relationship in weaker layers __without__ stomping the weaker opinions,
/// although stomping behavior is still possible, via set_targets().
///
/// An authored relationship creates a dependency of the targeting prim on
/// the targeted prim(s).  We consider these dependencies to be "load
/// dependencies", which means that when we load the targeting prim's
/// "load group", we will also load the targeted prims' load groups, to ensure
/// that all the data required to render the model containing the targeting
/// prim is composed and available.
///
/// Like [super::Attribute], [super::Relationship] objects are meant to
/// be ephemeral,
/// live on the stack, and be cheap to refetch from their owning UsdPrim.
///
/// Unlike [super::Attribute] s, which can either be uniform over all time
/// or vary in value over time, [Relationship] is __always uniform__.
impl Relationship {
    /// Returns true if any target path opinions have been authored.
    /// Note that this may include opinions that clear targets and may not
    /// indicate that target paths will exist for this relationship.
    pub fn has_authored_targets(&self) -> bool {
        unsafe {
            cpp!([self as "pxr::UsdRelationship*"]
                        -> bool as "bool" {
                return self->HasAuthoredTargets();
            })
        }
    }

    /// Compose this relationship's targets and fill `targets` with the result.
    /// All preexisting elements in `targets` are lost.
    ///
    /// Returns true if any target path opinions have been authored and no
    /// composition errors were encountered, returns false otherwise.
    /// Note that authored opinions may include opinions that clear the targets
    /// and a return value of true does not necessarily indicate that
    /// `targets`
    /// will contain any target paths.
    ///
    /// The result is not cached, so will be recomputed on every query.
    pub fn get_targets(&self, targets: &mut sdf::PathVector) {
        let targets = targets.as_mut();
        unsafe {
            cpp!([self as "pxr::UsdRelationship*",
                  targets as "pxr::SdfPathVector*"] {
                self->GetTargets(targets);
            })
        }
    }
}
