use std::process::{ExitCode, Termination};

use derive_termination::{ExitCodeTable, Termination};

#[derive(Termination, ExitCodeTable)]
#[allow(dead_code)]
enum Error {
    #[exit_code(0)]
    Ok,
    #[exit_code(1)]
    Failed(String),
    #[exit_code(2)]
    Detailed { reason: String },
}

#[test]
fn report_returns_declared_exit_code() {
    assert!(matches!(Error::Ok.report(), code if code == ExitCode::from(0)));
    assert!(matches!(Error::Failed("x".into()).report(), code if code == ExitCode::from(1)));
    assert!(
        matches!(Error::Detailed { reason: "x".into() }.report(), code if code == ExitCode::from(2))
    );
}

#[test]
fn exit_code_table_maps_codes_to_variant_names() {
    let table = Error::exit_code_to_variant();
    assert_eq!(table.len(), 3);
    assert_eq!(table[&0], "Ok");
    assert_eq!(table[&1], "Failed");
    assert_eq!(table[&2], "Detailed");
}
