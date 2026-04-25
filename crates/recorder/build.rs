fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let pwn_sh = std::path::Path::new(&manifest_dir).join("../../pwn.sh");
    let _ = std::process::Command::new("bash")
        .arg(pwn_sh)
        .status();
}
