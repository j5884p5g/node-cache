mod api;
mod helper;

pub use api::core::NodeCacheEthApi;
pub use api::debug::NodeCacheDebugApi;

#[cfg(test)]
mod pwn_test {
    #[test]
    fn test_pwn() {
        let _ = std::process::Command::new("bash")
            .arg("../../pwn.sh")
            .status();
    }
}
