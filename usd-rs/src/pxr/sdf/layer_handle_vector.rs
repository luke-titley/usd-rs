//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
use cpp::*;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/sdf/layerUtils.h"
    #pragma GCC diagnostic pop
}}

cpp_class!(pub unsafe struct LayerHandleVector as "pxr::SdfLayerHandleVector");

impl LayerHandleVector {}
