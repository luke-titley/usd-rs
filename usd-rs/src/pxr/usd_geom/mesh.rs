use crate::pxr::usd::Attribute;
use crate::pxr::usd::Prim;
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usdGeom/mesh.h"
    #include "pxr/usd/usdGeom/xformCache.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(
/// Encodes a mesh with optional subdivision properties and features.
///
/// As a point-based primitive, meshes are defined in terms of points that
/// are connected into edges and faces. Many references to meshes use the
/// term 'vertex' in place of or interchangeably with 'points', while some
/// use 'vertex' to refer to the 'face-vertices' that define a face.  To
/// avoid confusion, the term 'vertex' is intentionally avoided in favor of
/// 'points' or 'face-vertices'.
///
/// The connectivity between points, edges and faces is encoded using a
/// common minimal topological description of the faces of the mesh.  Each
/// face is defined by a set of face-vertices using indices into the Mesh's
/// _points_ array (inherited from UsdGeomPointBased) and laid out in a
/// single linear _faceVertexIndices_ array for efficiency.  A companion
/// _faceVertexCounts_ array provides, for each face, the number of
/// consecutive face-vertices in _faceVertexIndices_ that define the face.
/// No additional connectivity information is required or constructed, so
/// no adjacency or neighborhood queries are available.
    pub unsafe struct Mesh as "pxr::UsdGeomMesh"
);

impl Mesh {
    pub fn new(prim: &Prim) -> Mesh {
        unsafe {
            cpp!([prim as "pxr::UsdPrim*"]
                        -> Mesh as "pxr::UsdGeomMesh" {
                return pxr::UsdGeomMesh::Get(prim->GetStage(), prim->GetPath());
            })
        }
    }

    /// The primary geometry attribute for all PointBased
    /// primitives, describes points in (local) space.
    pub fn get_points_attribute(&self) -> Attribute {
        unsafe {
            cpp!([self as "pxr::UsdGeomMesh*"]
                        -> Attribute as "pxr::UsdAttribute" {
                return self->GetPointsAttr();
            })
        }
    }
}
