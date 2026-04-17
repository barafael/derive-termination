use derive_termination::Termination;

#[derive(Termination)]
enum Error {
    #[exit_code(256)]
    TooBig,
}

fn main() {}
