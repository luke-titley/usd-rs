//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use usd::pxr::sdf::*;
    use usd::pxr::tf;
    use usd::pxr::NoResult;

    #[test]
    fn test_get_instance() {
        Schema::get_instance();
        //Path::from("/root/world/stuff");
    }

    #[test]
    fn test_find_type() -> NoResult {
        use std::convert::TryFrom as _;

        let schema = Schema::get_instance();
        let _value_type_name = schema.find_type(&tf::Token::try_from("int")?);

        Ok(())
    }
}
