//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::pxr::sdf::*;

    #[test]
    fn test_get_instance() {
        Schema::get_instance();
        //Path::from("/root/world/stuff");
    }
}
