//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

#[repr(C)]
pub enum LoadPolicy {
    /// Load a prim plus all its descendants.
    UsdLoadWithDescendants = 0,

    /// Load a prim by itself with no descendants.
    UsdLoadWithoutDescendants = 1,
}
