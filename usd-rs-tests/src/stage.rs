//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use usd::pxr;
    use usd::pxr::sdf;
    use usd::pxr::tf;
    use usd::pxr::usd::*;
    use usd::pxr::vt;

    use std::convert::TryFrom;

    #[test]
    fn test_new() -> pxr::NoResult {
        let stage =
            Stage::create_new(stage::desc::CreateNew::from("test.usda"))?;
        stage.save();

        Ok(())
    }

    #[test]
    fn test_in_memory() -> pxr::NoResult {
        let stage =
            Stage::create_in_memory(stage::desc::CreateInMemory::default())?;
        stage.save();

        Ok(())
    }

    #[test]
    fn test_open() -> pxr::NoResult {
        let stage =
            Stage::create_new(stage::desc::CreateNew::from("test_open.usda"))?;
        stage.save();

        Stage::open(stage::desc::Open::from("test_open.usda"))?;

        Ok(())
    }

    #[test]
    fn test_define_prim() -> pxr::NoResult {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            "define_prim.usda",
        ))?;
        stage.define_prim(
            &sdf::Path::try_from("/root/world/test")?,
            &tf::Token::default(),
        );
        stage.save();

        Ok(())
    }

    #[test]
    fn test_create_attribute() -> pxr::NoResult {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            "create_attribute_prim.usda",
        ))?;
        let prim = stage.define_prim(
            &sdf::Path::try_from("/root/world/test").unwrap(),
            &tf::Token::default(),
        );

        prim.create_attribute(prim::desc::CreateAttribute {
            name: tf::Token::try_from("lukes_attr").unwrap(),
            type_name: sdf::Schema::get_instance()
                .find_type(&tf::Token::try_from("int").unwrap()),
        });

        stage.save();

        Ok(())
    }

    #[test]
    fn test_set_bool_attribute() -> pxr::NoResult {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            "set_attribute_prim.usda",
        ))?;
        let prim = stage.define_prim(
            &sdf::Path::try_from("/root/world/test")?,
            &tf::Token::default(),
        );

        let attr = prim.create_attribute(prim::desc::CreateAttribute {
            name: tf::Token::try_from("lukes_attr")?,
            type_name: sdf::Schema::get_instance()
                .find_type(&tf::Token::try_from("bool")?),
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

        Ok(())
    }

    #[test]
    fn test_set_string_attribute() -> pxr::NoResult {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            "set_string_attribute_prim.usda",
        ))?;
        let prim = stage.define_prim(
            &sdf::Path::try_from("/root/world/test").unwrap(),
            &tf::Token::default(),
        );

        let attr = prim.create_attribute(prim::desc::CreateAttribute {
            name: tf::Token::try_from("lukes_attr")?,
            type_name: sdf::Schema::get_instance()
                .find_type(&tf::Token::try_from("string")?),
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

        Ok(())
    }

    #[test]
    fn test_set_asset_path_attribute() -> pxr::NoResult {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            "set_asset_path_attribute_prim.usda",
        ))?;
        let prim = stage.define_prim(
            &sdf::Path::try_from("/root/world/test").unwrap(),
            &tf::Token::default(),
        );

        let attr = prim.create_attribute(prim::desc::CreateAttribute {
            name: tf::Token::try_from("lukes_attr")?,
            type_name: sdf::Schema::get_instance()
                .find_type(&tf::Token::try_from("asset")?),
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

        Ok(())
    }

    #[test]
    fn test_array() -> pxr::NoResult {
        use vt::VtArray as _;
        let mut array = vt::ArrayBool::new();

        array.reserve(2);
        array.push_back(&true);
        array.push_back(&false);

        assert_eq!(array.size(), 2_usize);
        assert_eq!(array[0], true);
        assert_eq!(array[1], false);

        let _value = vt::Value::from(&array);

        Ok(())
    }

    #[test]
    fn test_float_array_attribute_value() -> pxr::NoResult {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            "set_array_float_attribute_prim.usda",
        ))?;
        let prim = stage.define_prim(
            &sdf::Path::try_from("/root/world/test").unwrap(),
            &tf::Token::default(),
        );

        let attr = prim.create_attribute(prim::desc::CreateAttribute {
            name: tf::Token::try_from("lukes_attr")?,
            type_name: sdf::Schema::get_instance()
                .find_type(&tf::Token::try_from("float[]")?),
        });

        use vt::VtArray as _;
        let mut array = vt::ArrayFloat::new();
        array.push_back(&1.0);
        array.push_back(&2.0);
        array.push_back(&3.0);
        let mut _value = vt::Value::from(&array);

        attr.set(&_value, TimeCode::default());
        attr.get(&mut _value, TimeCode::default());

        stage.save();

        Ok(())
    }

    #[test]
    fn test_bool_array_attribute_value() -> pxr::NoResult {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            "set_array_bool_attribute_prim.usda",
        ))?;
        let prim = stage.define_prim(
            &sdf::Path::try_from("/root/world/test").unwrap(),
            &tf::Token::default(),
        );

        let attr = prim.create_attribute(prim::desc::CreateAttribute {
            name: tf::Token::try_from("lukes_attr")?,
            type_name: sdf::Schema::get_instance()
                .find_type(&tf::Token::try_from("bool[]")?),
        });

        use vt::VtArray as _;
        let mut array = vt::ArrayBool::new();
        array.push_back(&true);
        array.push_back(&false);
        array.push_back(&true);
        let mut _value = vt::Value::from(&array);

        attr.set(&_value, TimeCode::default());
        attr.get(&mut _value, TimeCode::default());

        stage.save();

        Ok(())
    }

    #[test]
    fn test_int_array_attribute_value() -> pxr::NoResult {
        let stage = Stage::create_new(stage::desc::CreateNew::from(
            "set_array_int_attribute_prim.usda",
        ))?;
        let prim = stage.define_prim(
            &sdf::Path::try_from("/root/world/test").unwrap(),
            &tf::Token::default(),
        );

        let attr = prim.create_attribute(prim::desc::CreateAttribute {
            name: tf::Token::try_from("lukes_attr")?,
            type_name: sdf::Schema::get_instance()
                .find_type(&tf::Token::try_from("int[]")?),
        });

        use vt::VtArray as _;
        let mut array = vt::ArrayInt::new();
        array.push_back(&1);
        array.push_back(&2);
        array.push_back(&3);
        let mut _value = vt::Value::from(&array);

        attr.set(&_value, TimeCode::default());
        attr.get(&mut _value, TimeCode::default());

        stage.save();

        Ok(())
    }
}
