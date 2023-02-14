//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//! The outermost container for scene description, which owns and presents
//! composed prims as a scenegraph, following the composition recipe recursively
//! described in its associated "root layer".
//!
//! USD derives its persistent-storage scalability by combining and reusing
//! simple compositions into richer aggregates using referencing and layering
//! with sparse overrides. Ultimately, every composition (i.e. "scene") is
//! identifiable by its root layer, i.e. the .usd file, and a scene is
//! instantiated in an application on a UsdStage that presents a composed view
//! of the scene's root layer. Each simple composition referenced into a larger
//! composition could be presented on its own UsdStage, at the same (or not)
//! time that it is participating in the larger composition on its own UsdStage;
//! all of the underlying layers will be shared by the two stages, while each
//! maintains its own scenegraph of composed prims.
//!
//! A UsdStage has sole ownership over the UsdPrim 's with which it is
//! populated, and retains shared ownership (with other stages and direct
//! clients of SdfLayer's, via the Sdf_LayerRegistry that underlies all SdfLayer
//! creation methods) of layers. It provides roughly five categories of API that
//! address different aspects of scene management:

//! - Stage lifetime management methods for constructing and initially populating
//! a UsdStage from an existing layer file, or one that will be created as a
//! result, in memory or on the filesystem.
//! - Load/unload working set management methods that allow you to specify which
//! payloads should be included and excluded from the stage's composition.
//! - Variant management methods to manage policy for which variant to use when
//! composing prims that provide a named variant set, but do not specify a
//! selection.
//! - Prim access, creation, and mutation methods that allow you to find, create,
//! or remove a prim identified by a path on the stage. This group also provides
//! methods for efficiently traversing the prims on the stage.
//! - Layers and EditTargets methods provide access to the layers in the stage's
//! root LayerStack (i.e. the root layer and all of its recursive sublayers),
//! and the ability to set a UsdEditTarget into which all subsequent mutations
//! to objects associated with the stage (e.g. prims, properties, etc) will go.
//! - Serialization methods for "flattening" a composition (to varying degrees),
//! and exporting a completely flattened view of the stage to a string or file.
//! These methods can be very useful for targetted asset optimization and
//! debugging, though care should be exercized with large scenes, as flattening
//! defeats some of the benefits of referenced scene description, and may
//! produce very large results, especially in file formats that do not support
//! data de-duplication, like the usda ASCII format!

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
pub mod stage_desc {
    use super::*;

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

    //------------------------------------------------------------------------------
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

    //------------------------------------------------------------------------------
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

    //------------------------------------------------------------------------------
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
cpp_class!(pub unsafe struct Stage as "pxr::UsdStageRefPtr");

//------------------------------------------------------------------------------
impl Stage {
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

    pub fn save(&self) -> pxr::NoResult {
        unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *"] {
                (*self)->Save();
            })
        };

        Ok(())
    }

    pub fn save_session_layers(&self) -> pxr::NoResult {
        unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *"] {
                (*self)->SaveSessionLayers();
            })
        };

        Ok(())
    }

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

    pub fn export(&self, file_path: &std::ffi::CStr) -> pxr::NoResult {
        let file_path = file_path.as_ptr() as *const std::os::raw::c_char;

        unsafe {
            cpp!([file_path as "const char *",
                  self as "const pxr::UsdStageRefPtr *"] {
                (*self)->Export(std::string(file_path));
            })
        };

        Ok(())
    }

    pub fn get_prim_at_path(&self, path: &sdf::Path) -> pxr::Result<Prim> {
        Ok(unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *",
                  path as "const pxr::SdfPath *"] -> Prim as "pxr::UsdPrim" {
                return (*self)->GetPrimAtPath(*path);
            })
        })
    }

    pub fn get_pseudo_root(&self) -> pxr::Result<Prim> {
        Ok(unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *"]
                    -> Prim as "pxr::UsdPrim" {
                return (*self)->GetPseudoRoot();
            })
        })
    }

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

    pub fn override_prim(&self, path: &sdf::Path) -> pxr::Result<Prim> {
        Ok(unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *",
                  path as "const pxr::SdfPath *"] -> Prim as "pxr::UsdPrim" {
                return (*self)->OverridePrim(*path);
            })
        })
    }

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

    pub fn get_session_layer(&self) -> sdf::LayerHandle {
        unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *"]
                    -> sdf::LayerHandle as "pxr::SdfLayerHandle" {
                return (*self)->GetSessionLayer();
            })
        }
    }

    pub fn get_root_layer(&self) -> sdf::LayerHandle {
        unsafe {
            cpp!([self as "const pxr::UsdStageRefPtr *"]
                    -> sdf::LayerHandle as "pxr::SdfLayerHandle" {
                return (*self)->GetRootLayer();
            })
        }
    }
}
