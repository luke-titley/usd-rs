use usd::pxr::usd;

fn main() {
    let path = std::ffi::CString::new("Spoon.geom.usd").unwrap();
    let stage = usd::Stage::open(usd::stage::desc::Open {
        file_path: &path,
        load: None,
    });

    for prim in stage.traverse().iter() {
        println!("Prim name: {:?}", prim.get_name().get_text());
    }
}
