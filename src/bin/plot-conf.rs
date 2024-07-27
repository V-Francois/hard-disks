use std::env;
use std::path;
use std::process;

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} path-to-position-file", args[0]);
        return process::ExitCode::from(1);
    }

    let file_path = &args[1];
    if !path::Path::new(file_path).exists() {
        eprintln!("Path {} doesn’t exist", file_path);
        return process::ExitCode::from(1);
    }
    println!("filepath: {}", file_path);

    process::ExitCode::SUCCESS
}
