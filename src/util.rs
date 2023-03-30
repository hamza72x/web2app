use std::collections::HashMap;
use std::fs;
use std::io;
use std::path;
use std::process;
use std::vec::Vec;

pub fn open_dir_in_explorer(dir: &String) {
    let cli_data: HashMap<&str, &str> = [
        ("windows", "explorer"),
        ("linux", "xdg-open"),
        ("macos", "open"),
    ]
    .iter()
    .cloned()
    .collect();

    process::Command::new(cli_data[std::env::consts::OS])
        .arg(dir)
        .spawn()
        .unwrap();
}

pub fn run_os_command_standard(args: &str, command_dir: Option<&str>) -> io::Result<()> {
    let args = args.split_whitespace().collect::<Vec<&str>>();

    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "No command provided"));
    }

    let mut command = process::Command::new(&args[0]);

    if command_dir.is_some() {
        command.current_dir(command_dir.unwrap());
    }

    for arg in args.iter().skip(1) {
        command.arg(arg);
    }

    command.status()?;

    Ok(())
}

pub fn re_create_dir(name: &str) -> io::Result<()> {
    let dir = path::Path::new(&name);

    // first delete the directory if it exists
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }

    fs::create_dir_all(dir)
}

pub fn copy_file(source: &String, dest: String) -> io::Result<()> {
    fs::copy(source, dest)?;
    Ok(())
}
