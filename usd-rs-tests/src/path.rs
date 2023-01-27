//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use usd::pxr::sdf::Path;

    #[test]
    fn test_from_str() {
        use std::convert::TryFrom as _;
        let _path = Path::try_from("/root/world/stuff").unwrap();
    }
}
