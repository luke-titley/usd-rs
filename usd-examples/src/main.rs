use usd::pxr;

fn open_kitchen_set() {
    let path = std::ffi::CString::new("../assets/Kitchen_set/Kitchen_set.usd").unwrap();
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

    let prim_path = std::ffi::CString::new("/root").unwrap();
    let prim = stage.define_prim(
        &pxr::sdf::Path::from(prim_path.as_c_str()),
        &pxr::tf::Token::default(),
    );

    prim.get_references()
        .add_reference(pxr::usd::references::desc::AddReference {
            identifier: &asset_path,
            prim_path: None,
        });
    stage.save();
}

fn main() {
    add_references();
}
