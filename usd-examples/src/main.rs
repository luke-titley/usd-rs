use usd::pxr;
use pxr::usd::Stage;
use pxr::sdf;
use std::ffi::CString;

use pxr::tf;
use pxr::usd::*;
use pxr::vt;

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
    let stage = Stage::create_new(stage::desc::CreateNew::from(
        CString::new("set_array_int_attribute_prim.usda")
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
            CString::new("int[]").unwrap().as_c_str(),
        )),
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
}

fn main() {
    array_attributes();
}
