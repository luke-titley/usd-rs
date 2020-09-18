//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[derive(Clone)]
#[repr(C)]
pub struct PointerAndBits {
    ptr_and_bits: *mut core::ffi::c_void,
}

static_assertions::const_assert_eq!(std::mem::size_of::<PointerAndBits>(), 8); // PointerAndBits size does not match
static_assertions::const_assert_eq!(std::mem::align_of::<PointerAndBits>(), 8); // PointerAndBits alignement does not match
