//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use usd::pxr::sdf::*;
    use usd::pxr::tf;

    #[test]
    fn test_get_instance() {
        Schema::get_instance();
        //Path::from("/root/world/stuff");
    }

    #[test]
    fn test_find_type() {
        let schema = Schema::get_instance();
        let _value_type_name = schema.find_type(&tf::Token::from(
            CString::new("int").unwrap().as_c_str(),
        ));
        //Path::from("/root/world/stuff");
    }
}
