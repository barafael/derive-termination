use derive_termination::ExitCodeTable;

#[derive(ExitCodeTable)]
enum Error {
    #[exit_code(0)]
    Ok,
    Missing,
}

fn main() {}
