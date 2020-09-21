//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::pxr::sdf;
    use crate::pxr::tf;
    use crate::pxr::usd::*;
    use crate::pxr::vt;
    use std::ffi::CString;

    #[test]
    fn test_new() {
        let stage = Stage::create_new(StageDescriptor::from("test.usda"));
        stage.save();
    }

    #[test]
    fn test_in_memory() {
        let stage = Stage::create_in_memory(StageInMemoryDescriptor::default());
        stage.save();
    }

    #[test]
    fn test_define_prim() {
        let stage =
            Stage::create_new(StageDescriptor::from("define_prim.usda"));
        stage.define_prim(
            &sdf::Path::from("/root/world/test"),
            &tf::Token::default(),
        );
        stage.save();
    }

    #[test]
    fn test_create_attribute() {
        let stage = Stage::create_new(StageDescriptor::from(
            "create_attribute_prim.usda",
        ));
        let prim = stage.define_prim(
            &sdf::Path::from("/root/world/test"),
            &tf::Token::default(),
        );

        prim.create_attribute(AttributeDescriptor {
            name: tf::Token::from(CString::new("lukes_attr").unwrap().as_c_str()),
            type_name: sdf::Schema::get_instance()
                .find_type(&tf::Token::from(CString::new("int").unwrap().as_c_str())),
        });

        stage.save();
    }

    #[test]
    fn test_set_attribute() {
        let stage =
            Stage::create_new(StageDescriptor::from("set_attribute_prim.usda"));
        let prim = stage.define_prim(
            &sdf::Path::from("/root/world/test"),
            &tf::Token::default(),
        );

        let attr = prim.create_attribute(AttributeDescriptor {
            name: tf::Token::from(CString::new("lukes_attr").unwrap().as_c_str()),
            type_name: sdf::Schema::get_instance()
                .find_type(&tf::Token::from(CString::new("bool").unwrap().as_c_str())),
        });

        attr.set(&vt::Value::from(<&vt::Bool>::from(&true)), TimeCode::default());

        let mut value = vt::Value::default();
        attr.get(&mut value, TimeCode::default());

        let result : &vt::Bool = value.as_ref();
        println!("The attribute value is {}", result.0 );

        stage.save();
    }
}
