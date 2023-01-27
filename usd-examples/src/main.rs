use pxr::sdf;
use pxr::usd::Stage;
use usd::pxr;

use pxr::tf;
use pxr::usd::*;
use pxr::vt;

use std::convert::TryFrom;

use usd::c_str;

fn open_kitchen_set() {
    let stage = pxr::usd::Stage::open(pxr::usd::stage::desc::Open {
        file_path: c_str!("../assets/Kitchen_set/Kitchen_set.usd"),
        load: None,
    });

    for prim in stage.traverse().iter() {
        println!("Prim path: {:?}", prim.get_path().get_text());
        println!("     type: {:?}", prim.get_type_name().get_text());
    }
}

fn add_references() {
    let asset_path = c_str!("asset.usda");
    let stage = pxr::usd::Stage::create_new(pxr::usd::stage::desc::CreateNew {
        identifier: asset_path,
        _load: None,
    });
    stage.define_prim(
        &pxr::sdf::Path::try_from("/root").unwrap(),
        &pxr::tf::Token::default(),
    );
    stage.save();

    let stage = pxr::usd::Stage::create_new(pxr::usd::stage::desc::CreateNew {
        identifier: c_str!("scene.usda"),
        _load: None,
    });

    stage
        .get_root_layer()
        .insert_sub_layer_path(asset_path, None);
    stage.save();
}

fn array_attributes() {
    let stage = Stage::create_new(pxr::usd::stage::desc::CreateNew {
        identifier: c_str!("set_array_int_attribute_prim.usda"),
        _load: None,
    });
    let prim = stage.define_prim(
        &sdf::Path::try_from("/root/world/test").unwrap(),
        &tf::Token::default(),
    );

    let attr = prim.create_attribute(prim::desc::CreateAttribute {
        name: tf::Token::from(c_str!("lukes_attr")),
        type_name: sdf::Schema::get_instance()
            .find_type(&tf::Token::from(c_str!("int[]"))),
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
