use std::process::ExitCode;

mod core;
mod print;

pub const RELEASE_DATE: &str = "2023-06-20";

fn main() -> ExitCode {
    print::show_title();
    match core::run() {
        Ok(msg) => {
            if msg.is_empty() {
                print::show_usage();
            } else {
                print::show_success(msg)
            }
            ExitCode::SUCCESS
        }
        Err(msg) => {
            print::show_error(msg);
            ExitCode::FAILURE
        }
    }
}
