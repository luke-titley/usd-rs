//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::pxr::sdf::path::*;

    #[test]
    fn test_from_str() {
        Path::from("/root/world/stuff");
    }
}
