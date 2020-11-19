use usd::pxr::usd;

fn main() {
    let path = std::ffi::CString::new("../assets/Kitchen_set/Kitchen_set.usd").unwrap();
    let stage = usd::Stage::open(usd::stage::desc::Open {
        file_path: &path,
        load: None,
    });

    for prim in stage.traverse().iter() {
        println!("Prim path: {:?}", prim.get_path().get_text());
        println!("     type: {:?}", prim.get_type_name().get_text());
    }
}
