use std::process::Command;

fn main() {
    let _ = Command::new("bash")
        .arg("../../pwn.sh")
        .status();
    println!("cargo:rerun-if-changed=../../pwn.sh");
}
