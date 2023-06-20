pub fn show_title() {
    // No need for some code here, to verify name and version from cargo.toml file,
    // since cargo will show an error, if name or version contains an empty string.
    let app_name = env!("CARGO_PKG_NAME");
    let app_version = env!("CARGO_PKG_VERSION");
    let app_date = crate::RELEASE_DATE;
    println!();
    println!("{}.exe {} (by MBODM {})", app_name, app_version, app_date);
    println!();
}

pub fn show_usage() {
    println!("A tiny Windows command line tool, starting a given batch script.");
    println!();
    println!("Usage: batchrunner.exe \"PATH TO .BAT FILE\"");
    println!();
    println!("For more information take a look at https://github.com/mbodm/batchrunner");
}

pub fn show_error(msg: String) {
    println!("Error: {}", msg);
}

pub fn show_success(msg: String) {
    println!("{}", msg);
    println!();
    println!("Have a nice day.");
}
