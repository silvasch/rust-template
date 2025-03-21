use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(e) = rust_bin_template::run() {
        eprintln!("error: {}", e);
        return ExitCode::from(1);
    }

    return ExitCode::SUCCESS;
}
