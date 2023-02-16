//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/stage.h"
    #pragma GCC diagnostic pop
}}

use crate::pxr;
use crate::pxr::sdf;
use crate::pxr::tf;

use crate::pxr::usd::prim::Prim;

use super::common::LoadPolicy;
use super::prim_range::*;

//------------------------------------------------------------------------------
#[repr(C)]
/// Specifies the initial set of prims to load when opening a UsdStage
pub enum InitialLoadSet {
    /// Load all loadable prims (default).
    LoadAll = 0,
    /// Load no loadable prims.
    LoadNone = 1,
}

impl Default for InitialLoadSet {
    fn default() -> Self {
        InitialLoadSet::LoadAll
    }
}

//------------------------------------------------------------------------------
/// Descriptors for [Stage] creation
pub mod stage_desc {
    use super::*;

    /// Parameters for creating a new [Stage] on disk
    pub struct CreateNew<'a> {
        pub identifier: &'a str,
        pub _load: Option<InitialLoadSet>,
    }

    impl<'a> From<&'a str> for CreateNew<'a> {
        fn from(identifier: &'a str) -> Self {
            Self {
                identifier,
                _load: None,
                // TODO : session_layer
                // TODO : path_resolver_context
            }
        }
    }

    /// Parameters for creating a new [Stage] in memory
    pub struct CreateInMemory {
        pub _load: Option<InitialLoadSet>,
    }

    impl From<InitialLoadSet> for CreateInMemory {
        fn from(load: InitialLoadSet) -> Self {
            Self { _load: Some(load) }
        }
    }

    impl Default for CreateInMemory {
        fn default() -> Self {
            Self { _load: None }
        }
    }

    /// Parameters for loading a stage [Stage]
    pub struct Load<'a> {
        pub path: Option<&'a sdf::Path>,
        pub policy: Option<LoadPolicy>,
    }

    impl<'a> Default for Load<'a> {
        fn default() -> Self {
            Self {
                path: None,
                policy: None,
            }
        }
    }

    /// Parameters for opening a new [Stage]
    pub struct Open<'a> {
        pub file_path: &'a str,
        pub load: Option<InitialLoadSet>,
        // path_resolver_context : ar::ResolverContext
    }

    impl<'a> From<&'a str> for Open<'a> {
        fn from(file_path: &'a str) -> Self {
            Self {
                file_path,
                load: None,
                // TODO : path_resolver_context
            }
        }
    }
}

//------------------------------------------------------------------------------
cpp_class!(
    /// The outermost container for scene description, which owns and presents
    /// composed prims as a scenegraph, following the composition recipe
    /// recursively described in its associated "root layer".
    ///
    /// USD derives its persistent-storage scalability by combining and reusing
    /// simple compositions into richer aggregates using referencing and
    /// layering with sparse overrides.  Ultimately, every composition
    /// (i.e. "scene") is identifiable by its root layer,
    /// i.e. the `.usd` file, and a scene is instantiated in an
    /// application on a UsdStage that presents a composed
    /// view of the scene's root layer.  Each simple composition referenced into
    /// a larger composition could be presented on its own UsdStage, at the same
    /// (or not) time that it is participating in the larger composition on its
    /// own UsdStage; all of the underlying layers will be shared by the two
    /// stages, while each maintains its own scenegraph of composed prims.
    pub unsafe struct Stage as "pxr::UsdStageRefPtr"
);

impl Stage {
    /// Create a new stage with root layer `identifier`, destroying
    /// potentially existing files with that identifier; it is considered an
    /// error if an existing, open layer is present with this identifier.
    ///
    /// Parameters are in [stage_desc::CreateNew]
    pub fn create_new<'a>(
        desc: stage_desc::CreateNew<'a>,
    ) -> pxr::Result<Self> {
        let identifier_str = std::ffi::CString::new(desc.identifier)?;

        let identifier_char =
            identifier_str.as_ptr() as *const std::os::raw::c_char;

        let result = unsafe {
            cpp!([identifier_char as "const char *"]
                -> Stage as "pxr::UsdStageRefPtr" {
                return pxr::UsdStage::CreateNew(std::string(identifier_char));
            })
        };

        Ok(result)
    }

    /// Creates a new stage only in memory, analogous to creating an
    /// anonymous SdfLayer.
    ///
    /// If `pathResolverContext` is provided it will be bound when creating the
    /// root layer at `identifier` and whenever asset path resolution is done
    /// for this stage, regardless of what other context may be bound at that
    /// time. Otherwise Usd will create the root layer with no context bound,
    /// then create a context for all future asset path resolution for the stage
    /// by calling ArResolver::CreateDefaultContext.
    ///
    /// The initial set of prims to load on the stage can be specified
    /// using the `load` parameter. [InitialLoadSet].
    pub fn create_in_memory(
        _desc: stage_desc::CreateInMemory,
    ) -> pxr::Result<Self> {
        let result = unsafe {
            cpp!([] -> Stage as "pxr::UsdStageRefPtr" {
                return pxr::UsdStage::CreateInMemory();
            })
        };

        Ok(result)
    }

    /// Attempt to find a matching existing stage in a cache if
    /// usd.StageCacheContext objects exist on the stack. Failing that, create a
    /// new stage and recursively compose prims defined within and referenced by
    /// the layer at `filePath`, which must already exist.
    ///
    /// The initial set of prims to load on the stage can be specified
    /// using the `load` parameter. [InitialLoadSet].
    pub fn open<'a>(desc: stage_desc::Open) -> pxr::Result<Self> {
        match desc {
            stage_desc::Open {
                file_path,
                load: None,
            } => {
                let path_str = std::ffi::CString::new(file_path)?;

                let file_path =
                    path_str.as_ptr() as *const std::os::raw::c_char;

                let result = unsafe {
                    cpp!([file_path as "const char *"] ->
                                Stage as "pxr::UsdStageRefPtr" {
                        return pxr::UsdStage::Open(std::string(file_path));
                    })
                };

                Ok(result)
            }
            stage_desc::Open {
                file_path,
                load: Some(load),
            } => {
                let path_str = std::ffi::CString::new(file_path)?;

                let file_path =
                    path_str.as_ptr() as *const std::os::raw::c_char;

                let result = unsafe {
                    cpp!([file_path as "const char *",
                        load as "pxr::UsdStage::InitialLoadSet"] ->
                                Stage as "pxr::UsdStageRefPtr" {
                        return pxr::UsdStage::Open(std::string(file_path), load);
                    })
                };

                Ok(result)
            }
        }
    }

    /// Functions for saving changes to layers that contribute opinions to
    /// this stage.  Layers may also be saved by calling Sdf.Layer::Save or
    /// exported to a new file by calling Sdf.Layer::Export.
    pub fn save(&self) -> pxr::NoResult {
        unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *"] {
                (*self)->Save();
            })
        };

        Ok(())
    }

    /// Calls Sdf.Layer::Save on all dirty session layers and sublayers of
    /// session layers contributing to this stage.
    ///
    /// This function will emit a warning and skip each dirty anonymous
    /// layer it encounters, since anonymous layers cannot be saved with
    /// Sdf.Layer::Save. These layers must be manually exported by calling
    /// Sdf.Layer::Export.
    pub fn save_session_layers(&self) -> pxr::NoResult {
        unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *"] {
                (*self)->SaveSessionLayers();
            })
        };

        Ok(())
    }

    /// Modify this stage's load rules to load the prim at `path`, its
    /// ancestors, and all of its descendants if `policy` is
    /// Policy::LoadWithDescendants.  If `policy` is
    /// Policy::LoadWithoutDescendants, then payloads on descendant prims are not loaded.
    pub fn load(&self, desc: stage_desc::Load) -> pxr::Result<Prim> {
        Ok(match desc {
            stage_desc::Load {
                path: None,
                policy: None,
            } => unsafe {
                cpp!([self as "const pxr::UsdStageRefPtr *"]
                        -> Prim as "pxr::UsdPrim" {
                    return (*self)->Load();
                })
            },
            stage_desc::Load {
                path: Some(path),
                policy: None,
            } => unsafe {
                cpp!([self as "const pxr::UsdStageRefPtr *",
                      path as "const pxr::SdfPath *"] -> Prim as "pxr::UsdPrim" {
                    return (*self)->Load(*path);
                })
            },
            stage_desc::Load {
                path: None,
                policy: Some(policy),
            } => unsafe {
                cpp!([self as "const pxr::UsdStageRefPtr *",
                      policy as "pxr::UsdLoadPolicy"] -> Prim as "pxr::UsdPrim" {
                    return (*self)->Load(pxr::SdfPath::AbsoluteRootPath(),
                                         policy);
                })
            },
            stage_desc::Load {
                path: Some(path),
                policy: Some(policy),
            } => unsafe {
                cpp!([self as "const pxr::UsdStageRefPtr *",
                      path as "const pxr::SdfPath *",
                      policy as "pxr::UsdLoadPolicy"] -> Prim as "pxr::UsdPrim" {
                    return (*self)->Load(*path, policy);
                })
            },
        })
    }

    /// Modify this stage's load rules to unload the prim and its descendants
    /// specified by `path`.
    pub fn unload(&self, path: Option<&sdf::Path>) -> pxr::NoResult {
        match path {
            None => unsafe {
                cpp!([self as "const pxr::UsdStageRefPtr *"] {
                    (*self)->Unload();
                })
            },
            Some(path) => unsafe {
                cpp!([self as "const pxr::UsdStageRefPtr *",
                      path as "const pxr::SdfPath *"] {
                    (*self)->Unload(*path);
                })
            },
        }

        Ok(())
    }

    /// Writes out the composite scene as a single flattened layer into
    /// `filename`.
    ///
    pub fn export(&self, file_path: &str) -> pxr::NoResult {
        let file_path_str = std::ffi::CString::new(file_path)?;
        let file_path = file_path_str.as_ptr() as *const std::os::raw::c_char;

        unsafe {
            cpp!([file_path as "const char *",
                  self as "const pxr::UsdStageRefPtr *"] {
                (*self)->Export(std::string(file_path));
            })
        };

        Ok(())
    }

    /// Return the [Prim] at `path`, or an invalid [Prim] if none exists.
    ///
    /// If `path` indicates a prim beneath an instance, returns an instance
    /// proxy prim if a prim exists at the corresponding path in that instance's
    /// prototype.
    ///
    /// Unlike override_prim() and define_prim(), this method will never author
    /// scene description, and therefore is safe to use as a "reader" in the Usd
    /// multi-threading model.
    pub fn get_prim_at_path(&self, path: &sdf::Path) -> pxr::Result<Prim> {
        Ok(unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *",
                  path as "const pxr::SdfPath *"] -> Prim as "pxr::UsdPrim" {
                return (*self)->GetPrimAtPath(*path);
            })
        })
    }

    /// Return the stage's "pseudo-root" prim, whose name is defined by Usd.
    ///
    /// The stage's named root prims are namespace children of this prim,
    /// which exists to make the namespace hierarchy a tree instead of a
    /// forest.  This simplifies algorithms that want to traverse all prims.
    ///
    /// A Stage always has a pseudo-root prim, unless there was an error
    /// opening or creating the stage, in which case this method returns
    /// an invalid [Prim].
    pub fn get_pseudo_root(&self) -> pxr::Result<Prim> {
        Ok(unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *"]
                    -> Prim as "pxr::UsdPrim" {
                return (*self)->GetPseudoRoot();
            })
        })
    }

    /// Traverse the active, loaded, defined, non-abstract prims on this stage
    /// depth-first.
    ///
    /// Returns a [PrimRange] , which allows low-latency
    /// traversal, with the ability to prune subtrees from traversal.  It
    /// is iterable, so in its simplest form, one can do:
    ///
    /// ```ignore
    /// for prim in stage.traverse()? {
    ///     println!("{}", prim.get_name()?.get_text()?);
    /// }
    /// ```
    pub fn traverse(&self) -> pxr::Result<PrimRange> {
        let prm_range = unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *"] -> *const PrmRange as "const pxr::UsdPrimRange*" {
                return new pxr::UsdPrimRange((*self)->Traverse());
            })
        };

        Ok(PrimRange {
            _prim_range: prm_range,
        })
    }

    /// Attempt to ensure a [Prim] at `path` exists on this stage.
    ///
    /// If a prim already exists at `path`, return it.  Otherwise author
    /// __sdf.PrimSpecs__ with __specifier__ == __sdf.SpecifierOver__ and empty
    /// __typeName__ at the current EditTarget to create this prim and any
    /// nonexistent ancestors, then return it.
    ///
    /// The given __path__ must be an absolute prim path that does not contain
    /// any variant selections.
    ///
    /// If it is impossible to author any of the necessary PrimSpecs, (for
    /// example, in case __path__ cannot map to the current usd.EditTarget's
    /// namespace) issue an error and return an invalid [Prim].
    ///
    /// If an ancestor of __path__ identifies an __inactive__ prim, author scene
    /// description as described above but return an invalid prim, since the
    /// resulting prim is descendant to an inactive prim.
    ///
    pub fn override_prim(&self, path: &sdf::Path) -> pxr::Result<Prim> {
        Ok(unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *",
                  path as "const pxr::SdfPath *"] -> Prim as "pxr::UsdPrim" {
                return (*self)->OverridePrim(*path);
            })
        })
    }

    /// Attempt to ensure a __Prim__ at `path` is defined (according to
    /// UsdPrim::IsDefined()) on this stage.
    ///
    /// If a prim at `path` is already defined on this stage and `typeName` is
    /// empty or equal to the existing prim's typeName, return that prim.
    /// Otherwise author an __sdf.PrimSpec__ with __specifier__ ==
    /// __sdf.SpecifierDef__ and `typeName` for the prim at `path` at the
    /// current EditTarget.  Author __sdf.PrimSpec__ s with `specifier` ==
    /// __sdf.SpecifierDef__ and empty typeName at the current EditTarget for any
    /// nonexistent, or existing but not __Defined__ ancestors.
    ///
    /// The given __path__ must be an absolute prim path that does not contain
    /// any variant selections.
    ///
    /// If it is impossible to author any of the necessary PrimSpecs (for
    /// example, in case __path__ cannot map to the current UsdEditTarget's
    /// namespace or one of the ancestors of `path` is inactive on the
    /// UsdStage), issue an error and return an invalid __Prim__.
    ///
    /// Note that this method may return a defined prim whose typeName does not
    /// match the supplied `typeName`, in case a stronger typeName opinion
    /// overrides the opinion at the current EditTarget.
    ///
    pub fn define_prim(
        &self,
        path: &sdf::Path,
        type_name: &tf::Token,
    ) -> pxr::Result<Prim> {
        Ok(unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *",
                  path as "const pxr::SdfPath *",
                  type_name as "const pxr::TfToken *"] -> Prim as "pxr::UsdPrim" {
                return (*self)->DefinePrim(*path, *type_name);
            })
        })
    }

    /// Author an __sdf.PrimSpec__ with __specifier__ == __sdf.SpecifierClass__ for
    /// the class at root prim path `path` at the current EditTarget.  The
    /// current EditTarget must have EditTarget::IsLocalLayer() == true.
    ///
    /// The given __path__ must be an absolute, root prim path that does not
    /// contain any variant selections.
    ///
    /// If a defined (Prim::IsDefined()) non-class prim already exists at
    /// `path`, issue an error and return an invalid Prim.
    ///
    /// If it is impossible to author the necessary PrimSpec, issue an error
    /// and return an invalid __Prim__.
    pub fn create_class_prim(
        &self,
        root_prim_path: &sdf::Path,
    ) -> pxr::Result<Prim> {
        Ok(unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *",
                root_prim_path as "const pxr::SdfPath *"] -> Prim as "pxr::UsdPrim" {
                return (*self)->CreateClassPrim(*root_prim_path);
            })
        })
    }

    /// Remove all scene description for the given `path` and its subtree
    /// __in the current usd.EditTarget__.
    ///
    /// This method does not do what you might initially think!  Calling this
    /// function will not necessarily cause the usd.Prim at `path` on this
    /// stage to disappear.  Completely eradicating a prim from a composition
    /// can be an involved process, involving edits to many contributing layers,
    /// some of which (in many circumstances) will not be editable by a client.
    /// This method is a surgical instrument that __can__ be used iteratively
    /// to effect complete removal of a prim and its subtree from namespace,
    /// assuming the proper permissions are acquired, but more commonly it
    /// is used to perform layer-level operations; e.g.: ensuring that a given
    /// layer (as expressed by a usd.EditTarget) provides no opinions for a
    /// prim and its subtree.
    ///
    /// Generally, if your eye is attracted to this method, you probably want
    /// to instead use usd.Prim::SetActive(false) , which will provide the
    /// "composed effect" of removing the prim and its subtree from the
    /// composition, without actually removing any
    /// scene description, which as a bonus, means that the effect is
    /// reversible at a later time!
    pub fn remove_prim(&self, path: &sdf::Path) -> pxr::NoResult {
        let result = unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *",
                  path as "const pxr::SdfPath *"] -> bool as "bool" {
                return (*self)->RemovePrim(*path);
            })
        };
        if result {
            Ok(())
        } else {
            Err(pxr::Error::UnableToRemovePrim)
        }
    }

    /// Return this stage's root session layer.
    pub fn get_session_layer(&self) -> sdf::LayerHandle {
        unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *"]
                    -> sdf::LayerHandle as "pxr::SdfLayerHandle" {
                return (*self)->GetSessionLayer();
            })
        }
    }

    /// Return this stage's root layer.
    pub fn get_root_layer(&self) -> sdf::LayerHandle {
        unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *"]
                    -> sdf::LayerHandle as "pxr::SdfLayerHandle" {
                return (*self)->GetRootLayer();
            })
        }
    }
}
