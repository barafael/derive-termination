# derive-termination
derive std::process::Termination for an enum.

````rust
use std::process::{ExitCode, Termination};
use derive_termination::Termination;

#[derive(Termination)]
pub enum Error {
    #[exit_code(3)]
    Fatal(bool, u8),

    #[exit_code(4)]
    Whatever { name: String },

    #[exit_code(5)]
    Anyhow,
}

#[test]
fn should_report_3() {
    assert_eq!(Error::Fatal(true, 4).report(), ExitCode::from(3));
}
````