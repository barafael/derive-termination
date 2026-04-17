use derive_termination::Termination;

#[derive(Termination)]
enum Error {
    #[exit_code(0)]
    Ok,
    Missing,
}

fn main() {}
