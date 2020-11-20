use usd::pxr;

fn open_kitchen_set() {
    let path = std::ffi::CString::new("../assets/Kitchen_set/Kitchen_set.usd")
        .unwrap();
    let stage = pxr::usd::Stage::open(pxr::usd::stage::desc::Open {
        file_path: &path,
        load: None,
    });

    for prim in stage.traverse().iter() {
        println!("Prim path: {:?}", prim.get_path().get_text());
        println!("     type: {:?}", prim.get_type_name().get_text());
    }
}

fn add_references() {
    let asset_path = std::ffi::CString::new("asset.usda").unwrap();
    let stage = pxr::usd::Stage::create_new(pxr::usd::stage::desc::CreateNew {
        identifier: &asset_path,
        _load: None,
    });
    let prim_path = std::ffi::CString::new("/root").unwrap();
    let prim = stage.define_prim(
        &pxr::sdf::Path::from(prim_path.as_c_str()),
        &pxr::tf::Token::default(),
    );
    stage.save();

    let scene_path = std::ffi::CString::new("scene.usda").unwrap();
    let stage = pxr::usd::Stage::create_new(pxr::usd::stage::desc::CreateNew {
        identifier: &scene_path,
        _load: None,
    });

    stage
        .get_root_layer()
        .insert_sub_layer_path(&asset_path, None);
    stage.save();
}

fn array_attributes() {
    use usd::pxr::vt::VtArray as _;
    let mut array = usd::pxr::vt::ArrayInt::new();
    array.push_back(&123_i32);
    let value = usd::pxr::vt::Value::from(&array);

    let asset_path = std::ffi::CString::new("array_value.usda").unwrap();
    let stage = pxr::usd::Stage::create_new(pxr::usd::stage::desc::CreateNew {
        identifier: &asset_path,
        _load: None,
    });
    let prim_path = std::ffi::CString::new("/root").unwrap();
    let prim = stage.define_prim(
        &pxr::sdf::Path::from(prim_path.as_c_str()),
        &pxr::tf::Token::default(),
    );

    let attr =
        prim.create_attribute(usd::pxr::usd::prim::desc::CreateAttribute {
            name: usd::pxr::tf::Token::from(
                std::ffi::CString::new("lukes_attr").unwrap().as_c_str(),
            ),
            type_name: usd::pxr::sdf::Schema::get_instance().find_type(
                &usd::pxr::tf::Token::from(
                    std::ffi::CString::new("int[]").unwrap().as_c_str(),
                ),
            ),
        });

    attr.set(&value, usd::pxr::usd::TimeCode::default());

    stage.save();
}

fn main() {
    array_attributes();
}
