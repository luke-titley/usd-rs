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

impl std::clone::Clone for TfRefPtr {
    fn clone(&self) -> Self {
        let new_ref_base = unsafe { self.ref_base.clone() };
        Self {
            ref_base: new_ref_base,
        }
    }
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

    pub fn export(&self) {
        let data = self.this.clone();
        unsafe {
            cpp!([data as "pxr::UsdStageRefPtr"] {
                data->Export("test_out.usda");
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage() {
        let stage = Stage::create_in_memory();
        stage.export();
    }
}
