use std::collections::HashMap;
use std::fs;
use std::io;
use std::path;
use std::process;
use std::vec::Vec;

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
    ]
    .iter()
    .cloned()
    .collect();

    process::Command::new(cli_data[std::env::consts::OS])
        .arg(dir)
        .spawn()
        .unwrap();
}

pub fn run_os_command(args: &str, command_dir: Option<&str>) -> Result<String, String> {
    let args = args.split_whitespace().collect::<Vec<&str>>();

    if args.is_empty() {
        return Err(String::from("No command given"));
    }

    let mut command = process::Command::new(&args[0]);

    if command_dir.is_some() {
        command.current_dir(command_dir.unwrap());
    }

    for arg in args.iter().skip(1) {
        command.arg(arg);
    }

    let output = command
        .output()
        .map_err(|err| format!("Error executing command: {}", err));

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8(output.stdout).unwrap())
            } else {
                Err(String::from_utf8(output.stderr).unwrap())
            }
        }
        Err(err) => Err(err),
    }        
}

pub fn re_create_dir(name: &str) -> io::Result<()> {
    let dir = path::Path::new(&name);

    // first delete the directory if it exists
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }

    fs::create_dir_all(dir)
}

// base64 decode
// returns string
pub fn decode_base64(data: &str) -> String {
    let decoded = base64::decode(data).unwrap();
    String::from_utf8(decoded).unwrap()
}
