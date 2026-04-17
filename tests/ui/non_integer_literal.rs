use derive_termination::Termination;

#[derive(Termination)]
enum Error {
    #[exit_code("one")]
    Stringy,
}

fn main() {}
