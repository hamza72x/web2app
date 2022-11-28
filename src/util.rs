use std::collections::HashMap;
use std::vec::Vec;
use std::io;
use std::path;
use std::fs;
use std::process;

pub fn get_home_dir() -> String {
    home::home_dir().unwrap().display().to_string()
}

pub fn resize_icon(icon_path: &String, size: u8, new_path: String) -> io::Result<()> {
    let mut convert = process::Command::new("convert");
    convert.arg(icon_path);
    convert.arg("-resize");
    convert.arg(format!("{}x{}", size, size));
    convert.arg(new_path);
    convert.status()?;

    Ok(())
}

pub fn open_dir_in_explorer(dir: &String) {

    let cli_data: HashMap<&str, &str> = [
        ("windows", "explorer"),
        ("linux", "xdg-open"),
        ("macos", "open"),
    ].iter().cloned().collect();

     process::Command::new(cli_data[std::env::consts::OS])
     .arg(dir)
     .spawn()
     .unwrap();
}

pub fn run_os_command(args: &str, command_dir: Option<&str>) -> io::Result<()> {
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
