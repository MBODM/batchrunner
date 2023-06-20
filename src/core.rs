use std::{env, ffi::OsStr, os::windows::process::CommandExt, path::Path, process::Command};

const ERR_MSG_1: &str = "Too many arguments.";
const ERR_MSG_2: &str = "Given file not exists.";
const ERR_MSG_3: &str = "Given file is not a batch script (.bat) file.";

pub fn run() -> Result<String, String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args_len = args.len();
    if args_len == 0 {
        return Ok(String::from(""));
    }
    if args_len > 1 {
        return Err(String::from(ERR_MSG_1));
    }
    let batch = &args[0];
    let path = Path::new(batch);
    if !path.exists() {
        return Err(String::from(ERR_MSG_2));
    }
    if path.extension() != Some(OsStr::new("bat")) {
        return Err(String::from(ERR_MSG_3));
    }
    Command::new("cmd.exe")
        .arg("/c")
        .raw_arg(format!("\"{}\"", batch)) // This is important, since Rust + cmd.exe do weird escaping stuff!
        .spawn()
        .map_err(|err| err.to_string())?;

    let msg_head = "Successfully started batch script";
    let file_name = match path.file_name() {
        Some(s) => 
        None => String::from(format!("{} batch script", msg_head))
    };
    Ok(msg)
}
