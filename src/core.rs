use std::{env, ffi::OsStr, os::windows::process::CommandExt, path::Path, process::Command};

pub fn run() -> Result<String, String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() == 0 {
        return Ok(String::new());
    }
    if args.len() >= 2 {
        return Err(String::from("Too many arguments."));
    }
    let bat = &args[0];
    let path = Path::new(bat);
    validate_argument(path)?;
    run_bat(bat)?;
    let name = get_file_name(path)?;
    let msg = format!("{} successfully started", name);
    Ok(msg)
}

fn validate_argument(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(String::from("Given file not exists."));
    }
    if !path.is_file() {
        return Err(String::from("Given path is not a file."));
    }
    if path.extension() != Some(OsStr::new("bat")) {
        return Err(String::from("Given file is not a .bat file."));
    }
    Ok(())
}

fn get_file_name(path: &Path) -> Result<String, String> {
    const ERR_MSG: &str = "Could not determine file name.";
    let str = path.file_name().ok_or(ERR_MSG)?.to_str().ok_or(ERR_MSG)?;
    let s = String::from(str);
    Ok(s)
}

fn run_bat(bat: &str) -> Result<(), String> {
    Command::new("cmd.exe")
        .arg("/c")
        .raw_arg(format!("\"{}\"", bat)) // This is important, since Rust + cmd.exe do weird escaping stuff!
        .spawn()
        .map_err(|err| err.to_string())?;
    Ok(())
}
