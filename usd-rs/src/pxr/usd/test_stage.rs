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
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            CString::new("test.usda").unwrap().as_c_str(),
        ));
        stage.save();
    }

    #[test]
    fn test_in_memory() {
        let stage =
            Stage::create_in_memory(stage::desc::CreateInMemory::default());
        stage.save();
    }

    #[test]
    fn test_define_prim() {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            CString::new("define_prim.usda").unwrap().as_c_str(),
        ));
        let path = CString::new("/root/world/test").unwrap();
        stage.define_prim(
            &sdf::Path::from(path.as_c_str()),
            &tf::Token::default(),
        );
        stage.save();
    }

    #[test]
    fn test_create_attribute() {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            CString::new("create_attribute_prim.usda")
                .unwrap()
                .as_c_str(),
        ));
        let path = CString::new("/root/world/test").unwrap();
        let prim = stage.define_prim(
            &sdf::Path::from(path.as_c_str()),
            &tf::Token::default(),
        );

        prim.create_attribute(prim::desc::CreateAttribute {
            name: tf::Token::from(
                CString::new("lukes_attr").unwrap().as_c_str(),
            ),
            type_name: sdf::Schema::get_instance().find_type(&tf::Token::from(
                CString::new("int").unwrap().as_c_str(),
            )),
        });

        stage.save();
    }

    #[test]
    fn test_set_bool_attribute() {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            CString::new("set_attribute_prim.usda").unwrap().as_c_str(),
        ));
        let path = CString::new("/root/world/test").unwrap();
        let prim = stage.define_prim(
            &sdf::Path::from(path.as_c_str()),
            &tf::Token::default(),
        );

        let attr = prim.create_attribute(prim::desc::CreateAttribute {
            name: tf::Token::from(
                CString::new("lukes_attr").unwrap().as_c_str(),
            ),
            type_name: sdf::Schema::get_instance().find_type(&tf::Token::from(
                CString::new("bool").unwrap().as_c_str(),
            )),
        });

        attr.set(
            &vt::Value::from(<&vt::Bool>::from(&true)),
            TimeCode::default(),
        );

        let mut value = vt::Value::default();
        attr.get(&mut value, TimeCode::default());

        let result: &vt::Bool = value.as_ref();
        println!("The attribute value is {}", result.0);

        stage.save();
    }

    #[test]
    fn test_set_string_attribute() {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            CString::new("set_string_attribute_prim.usda")
                .unwrap()
                .as_c_str(),
        ));
        let path = CString::new("/root/world/test").unwrap();
        let prim = stage.define_prim(
            &sdf::Path::from(path.as_c_str()),
            &tf::Token::default(),
        );

        let attr = prim.create_attribute(prim::desc::CreateAttribute {
            name: tf::Token::from(
                CString::new("lukes_attr").unwrap().as_c_str(),
            ),
            type_name: sdf::Schema::get_instance().find_type(&tf::Token::from(
                CString::new("string").unwrap().as_c_str(),
            )),
        });

        attr.set(
            &vt::Value::from(<&vt::String>::from(
                CString::new("this is a string").unwrap().as_c_str(),
            )),
            TimeCode::default(),
        );

        let mut value = vt::Value::default();
        attr.get(&mut value, TimeCode::default());

        let result: &vt::String = value.as_ref();
        println!("The attribute value is {}", result.0.to_str().unwrap());

        stage.save();
    }

    #[test]
    fn test_set_asset_path_attribute() {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            CString::new("set_asset_path_attribute_prim.usda")
                .unwrap()
                .as_c_str(),
        ));
        let path = CString::new("/root/world/test").unwrap();
        let prim = stage.define_prim(
            &sdf::Path::from(path.as_c_str()),
            &tf::Token::default(),
        );

        let attr = prim.create_attribute(prim::desc::CreateAttribute {
            name: tf::Token::from(
                CString::new("lukes_attr").unwrap().as_c_str(),
            ),
            type_name: sdf::Schema::get_instance().find_type(&tf::Token::from(
                CString::new("asset").unwrap().as_c_str(),
            )),
        });

        let path = CString::new("/root/show/asset.abc").unwrap();
        attr.set(
            &vt::Value::from(<&vt::Asset>::from(
                sdf::AssetPath::new(sdf::AssetPathDescriptor {
                    path: path.as_c_str(),
                    resolved_path: None,
                })
                .as_ref(),
            )),
            TimeCode::default(),
        );

        let mut value = vt::Value::default();
        attr.get(&mut value, TimeCode::default());

        let result: &vt::Asset = value.as_ref();
        println!(
            "The attribute value is '{}'",
            result.0.get_asset_path().to_str().unwrap()
        );

        stage.save();
    }
}
