use pxr::sdf;
use pxr::usd::Stage;
use usd::pxr;

use pxr::tf;
use pxr::usd::*;
use pxr::vt;

use std::convert::TryFrom;

use usd::c_str;

fn open_kitchen_set() -> pxr::NoResult {
    let stage = pxr::usd::Stage::open(pxr::usd::stage::desc::Open {
        file_path: "../assets/Kitchen_set/Kitchen_set.usd",
        load: None,
    })?;

    for prim in stage.traverse().iter() {
        println!("Prim path: {:?}", prim.get_path().get_text());
        println!("     type: {:?}", prim.get_type_name().get_text());
    }

    Ok(())
}

fn add_references() -> pxr::NoResult {
    let asset_path = "asset.usda";

    let stage =
        pxr::usd::Stage::create_new(pxr::usd::stage::desc::CreateNew {
            identifier: asset_path,
            _load: None,
        })?;
    stage.define_prim(
        &pxr::sdf::Path::try_from("/root").unwrap(),
        &pxr::tf::Token::default(),
    );
    stage.save();

    let stage =
        pxr::usd::Stage::create_new(pxr::usd::stage::desc::CreateNew {
            identifier: "scene.usda",
            _load: None,
        })?;

    /*
    stage
        .get_root_layer()
        .insert_sub_layer_path(asset_path, None);
    */
    stage.save();

    Ok(())
}

fn array_attributes() -> pxr::NoResult {
    let stage = Stage::create_new(pxr::usd::stage::desc::CreateNew {
        identifier: "set_array_int_attribute_prim.usda",
        _load: None,
    })?;
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

    stage.save()?;

    Ok(())
}

fn main() {
    array_attributes().unwrap();
}
