use std::process::ExitCode;

mod application;

fn main() -> ExitCode {
    match application::run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
