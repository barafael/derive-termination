# derive-termination

Derive the `std::process::Termination` trait for an enum (annotate the variants with `#[exit_code(n)]`).

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

fn main() {
    assert_eq!(Error::Fatal(true, 4).report(), ExitCode::from(3));
}
````

The `Termination` derive macro above expands to:

````rust
pub enum Error {
    Fatal(bool, u8),
    Whatever { name: String },
    Anyhow,
}

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

Every variant must carry an `#[exit_code(N)]` attribute; a missing attribute is a compile-time error that names the offending variant.

## `ExitCodeTable`

Pair `Termination` with `ExitCodeTable` to get a static map from exit codes to variant names — useful for looking up a translation key, printing a reference table, or resolving an exit code back to a name.

````rust
use derive_termination::{ExitCodeTable, Termination};

#[derive(Termination, ExitCodeTable)]
pub enum Error {
    #[exit_code(0)] Ok,
    #[exit_code(1)] Failed(String),
}

fn main() {
    let table = Error::exit_code_to_variant();
    assert_eq!(table[&1], "Failed");
}
````

`exit_code_to_variant` returns `std::collections::BTreeMap<u8, &'static str>`.

## Changelog

### 2.0.0 (breaking)

- Missing `#[exit_code(N)]` on a variant is now a hard error with a clear message (previously: silently skipped, which led to a confusing `non-exhaustive match` compile error in generated code).
- New `ExitCodeTable` derive emitting `fn exit_code_to_variant() -> BTreeMap<u8, &'static str>`.

The `std::process::Termination` trait marks any type which is allowed to be returned from `main`.
