//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//! This contains everything you need for working with a usd stage, the main
//! point of entry into the usd library.

use cpp::cpp;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/stage.h"
    #pragma GCC diagnostic pop

}}

//------------------------------------------------------------------------------
type TfRefBase = core::ffi::c_void;

// This is really just a typedef.
#[derive(Clone)]
struct TfRefPtr {
    ref_base: *mut TfRefBase,
}

//------------------------------------------------------------------------------
#[repr(C)]
/// Specifies the initial set of prims to load when opening a UsdStage
pub enum InitialLoadSet {
    /// Load all loadable prims (default).
    LoadAll = 0,
    /// Load no loadable prims.
    LoadNone = 1,
}

//------------------------------------------------------------------------------
pub struct Descriptor<'a> {
    _identifier : &'a str,
    _load : Option<InitialLoadSet>,
}

impl<'a> From<&'a str> for Descriptor<'a> {
    fn from(identifier : &'a str) -> Self {
        Self {
            _identifier : identifier,
            _load : None,
            // TODO : session_layer
            // TODO : path_resolver_context
        }
    }
}

//------------------------------------------------------------------------------
pub struct InMemoryDescriptor {
    _load : Option<InitialLoadSet>,
}

impl From<InitialLoadSet> for InMemoryDescriptor {
    fn from(load : InitialLoadSet) -> Self {
        Self {
            _load : Some(load),
        }
    }
}

impl Default for InMemoryDescriptor {
    fn default() -> Self {
        Self {
            _load : None,
        }
    }
}

//------------------------------------------------------------------------------
/// The outermost container for scene description, which owns and presents
/// composed prims as a scenegraph, following the composition recipe recursively
/// described in its associated "root layer".
///
/// USD derives its persistent-storage scalability by combining and reusing
/// simple compositions into richer aggregates using referencing and layering
/// with sparse overrides. Ultimately, every composition (i.e. "scene") is
/// identifiable by its root layer, i.e. the .usd file, and a scene is
/// instantiated in an application on a UsdStage that presents a composed view
/// of the scene's root layer. Each simple composition referenced into a larger
/// composition could be presented on its own UsdStage, at the same (or not)
/// time that it is participating in the larger composition on its own UsdStage;
/// all of the underlying layers will be shared by the two stages, while each
/// maintains its own scenegraph of composed prims.
///
/// A UsdStage has sole ownership over the UsdPrim 's with which it is
/// populated, and retains shared ownership (with other stages and direct
/// clients of SdfLayer's, via the Sdf_LayerRegistry that underlies all SdfLayer
/// creation methods) of layers. It provides roughly five categories of API that
/// address different aspects of scene management:

/// - Stage lifetime management methods for constructing and initially populating
/// a UsdStage from an existing layer file, or one that will be created as a
/// result, in memory or on the filesystem.
/// - Load/unload working set management methods that allow you to specify which
/// payloads should be included and excluded from the stage's composition.
/// - Variant management methods to manage policy for which variant to use when
/// composing prims that provide a named variant set, but do not specify a
/// selection.
/// - Prim access, creation, and mutation methods that allow you to find, create,
/// or remove a prim identified by a path on the stage. This group also provides
/// methods for efficiently traversing the prims on the stage.
/// - Layers and EditTargets methods provide access to the layers in the stage's
/// root LayerStack (i.e. the root layer and all of its recursive sublayers),
/// and the ability to set a UsdEditTarget into which all subsequent mutations
/// to objects associated with the stage (e.g. prims, properties, etc) will go.
/// - Serialization methods for "flattening" a composition (to varying degrees),
/// and exporting a completely flattened view of the stage to a string or file.
/// These methods can be very useful for targetted asset optimization and
/// debugging, though care should be exercized with large scenes, as flattening
/// defeats some of the benefits of referenced scene description, and may
/// produce very large results, especially in file formats that do not support
/// data de-duplication, like the usda ASCII format!
pub struct Stage {
    this: TfRefPtr,
}

//------------------------------------------------------------------------------
impl Stage {
    pub fn create_new<'a>(_desc : Descriptor<'a>){}

    pub fn create_in_memory(_desc : InMemoryDescriptor) -> Self {
        let this = unsafe {
            cpp!([] -> TfRefPtr as "pxr::UsdStageRefPtr" {
                return pxr::UsdStage::CreateInMemory();
            })
        };

        Self { this }
    }

    pub fn export(&self) {
        let data = self.this.clone();
        unsafe {
            cpp!([data as "pxr::UsdStageRefPtr"] {
                data->Export("test_out.usda");
            })
        }
    }
}