//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use usd::pxr::sdf::Path;
    use std::ffi::CString;

    #[test]
    fn test_from_str() {
        let path_str = CString::new("/root/world/stuff").unwrap();
        let _path = Path::from(path_str.as_c_str());
    }
}
