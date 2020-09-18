fn main() {

    let stage = usd::Stage::create_in_memory();
    stage.export();

    println!("Hello, world!");
}
