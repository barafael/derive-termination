# derive-termination
derive std::process::Termination for an enum.

````rust
use std::process::{ExitCode, Termination};

use derive_termination::Termination;
use thiserror::Error;

pub struct Vec {
    b: bool,
}

#[test]
fn should_report_3() {
    assert_eq!(Error::Fatal(true, 4).report(), ExitCode::from(3));
}

#[derive(Debug, Termination, Error, Default)]
pub enum Error {
    #[exit_code(3)]
    #[error("Error fatale")]
    Fatal(bool, u8),

    #[exit_code(4)]
    #[error("ah man whatevvs")]
    Whatever { name: String },

    #[exit_code(5)]
    #[error("how any?")]
    #[default]
    Anyhow,
}
````