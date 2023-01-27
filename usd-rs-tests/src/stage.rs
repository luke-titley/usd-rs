use usd::pxr;
use usd::pxr::usd::*;
use usd::pxr::tf;
use usd::pxr::sdf;
use usd::pxr::vt;
use usd::c_str;

#[test]
fn array_attributes() {
    let stage = Stage::create_new(pxr::usd::stage::desc::CreateNew {
        identifier: c_str!("set_array_int_attribute_prim.usda"),
        _load: None,
    });
    let path = c_str!("/root/world/test");
    let prim = stage.define_prim(&sdf::Path::from(path), &tf::Token::default());

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
