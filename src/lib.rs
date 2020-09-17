use cpp::cpp;

cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usd/stage.h"
    #pragma GCC diagnostic pop

}}

type TfRefBase = core::ffi::c_void;

// This is really just a typedef.
struct TfRefPtr {
    ref_base: *mut TfRefBase,
}

// We represent the usd stage as a reference directly.
pub struct Stage {
    this: TfRefPtr,
}

type void = core::ffi::c_void;

impl Stage {
    pub fn create_in_memory() -> Self {
        let this = unsafe {
            cpp!([] -> TfRefPtr as "pxr::UsdStageRefPtr" {
                return pxr::UsdStage::CreateInMemory();
            })
        };

        Self { this }
    }
}

/*
pub fn test ()
{
    let mut y = 2;
    let mut z = 45;
    let x: i32 = unsafe { cpp!([y as "int32_t", mut z as "int32_t"] -> i32 as "int32_t" {
        z++;
        return y + z;
    })};
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage() {
        let stage = Stage::create_in_memory();
    }
}
