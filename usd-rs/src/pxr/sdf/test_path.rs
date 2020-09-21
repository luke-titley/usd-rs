//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::pxr::sdf::path::*;
    use std::ffi::CString;

    #[test]
    fn test_from_str() {
        let path = CString::new("/root/world/stuff").unwrap();
        Path::from(path.as_c_str());
    }
}
