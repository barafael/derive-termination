# derive-termination

Derive the std::process::Termination trait for an enum (annotate the variants with `#[exit_code(n)`).

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

The `Termination` derive macro above would generate:

````rust
impl ::std::process::Termination for Error {
    fn report(self) -> ::std::process::ExitCode {
        match self {
            Self::Fatal(..) => ::std::process::ExitCode::from(3),
            Self::Whatever { .. } => ::std::process::ExitCode::from(4),
            Self::Anyhow => ::std::process::ExitCode::from(5),
        }
    }
}
````

The `std::process::Termination` trait is a trait marking any type which is allowed to be returned from `main`.