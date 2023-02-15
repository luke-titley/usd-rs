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
cpp_class!(
    pub unsafe struct Stage as "pxr::UsdStageRefPtr"
);

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
