//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/primRange.h"
    #pragma GCC diagnostic pop
}}

//------------------------------------------------------------------------------
cpp_class!(pub unsafe struct PrimRange as "pxr::UsdPrimRange");

impl PrimRange {}