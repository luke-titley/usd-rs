use std::process::Command;

fn main() {
    let output = Command::new("echo")
        .arg("Hello world")
        .output()
        .expect("Failed to execute command");


    assert_eq!(b"Hello world\n", output.stdout.as_slice());
}
