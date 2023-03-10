use std::fs;
use std::io;
use std::io::Write;
use std::path;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;

mod generated;
mod input;
mod model;
mod util;

// milliseconds
const SLEEP_TIME: u64 = 10;
const ICON_SIZE_1: u8 = 32;
const ICON_SIZE_2: u8 = 128;

fn main() -> io::Result<()> {
    let cli = model::Cli::parse();
    let data: model::AppData;

    match cli.command {
        Some(model::Commands::Args(arg_data)) => {
            data = arg_data;
        }
        Some(model::Commands::Interactive) => {
            data = get_interactive_data();
        }
        None => {
            println!("No command given. Use --help for more information.");
            return Err(io::Error::new(io::ErrorKind::Other, "No command given."));
        }
    }

    // panics if fails
    check_pre_requisites();

    // print given input
    data.print();
    sleep(Duration::from_millis(SLEEP_TIME));

    // building
    build(&data)?;
    sleep(Duration::from_millis(SLEEP_TIME));

    // opening output directory in file explorer
    util::open_dir_in_explorer(&data.bundle_dir());

    Ok(())
}

fn get_interactive_data() -> model::AppData {
    let mut data: model::AppData = model::AppData {
        name: String::from("TestApp"),
        url: String::from("https://notion.so"),
        description: String::from("An example application."),
        version: String::from("0.1.0"),
        author: String::from("John Doe"),
        identifier: String::from("com.example.testapp"),
        icon: None,
        release_build: true,
    };

    data.name = input::string_must("Name");
    data.url = input::string_must("URL");
    data.description = input::string("Description", "An example application.");
    data.version = input::string("Version", "0.1.0");
    data.author = input::string("Author", "John Doe");
    data.identifier = input::string("Identifier", "com.example.testapp");
    data.icon = input::optional_string("Icon", "icon_path.png");
    data.release_build = input::bool("Release build", true);

    return data;
}

// build the app
fn build(data: &model::AppData) -> io::Result<()> {
    // build directories
    print_and_wait("\n🎉 Building directories...");

    fs::create_dir_all(&data.build_dir())?;
    util::re_create_dir(&data.src_dir())?;

    // create files
    print_and_wait("\n🎉 Creating files...");

    let mut cargo_toml = fs::File::create(&path::PathBuf::from(&data.cargo_toml_path())).unwrap();
    let mut main_rs = fs::File::create(&path::PathBuf::from(&data.main_rs_path())).unwrap();

    print_and_wait("\n🎉 Writing to files...");

    let template_main_rs = util::decode_base64(generated::MAIN_RS);
    let template_cargo_toml = util::decode_base64(generated::CARGO_TOML);

    main_rs
        .write_all(build_template(template_main_rs, &data).as_bytes())
        .unwrap();
    cargo_toml
        .write_all(build_template(template_cargo_toml, &data).as_bytes())
        .unwrap();

    // build icons
    print_and_wait("\n🎉 Building icons...");

    if data.icon.is_some() {
        let source_icon = data.icon.as_ref().unwrap();
        util::resize_icon(&source_icon, ICON_SIZE_1, data.icon_path(ICON_SIZE_1)).unwrap();
        util::resize_icon(&source_icon, ICON_SIZE_2, data.icon_path(ICON_SIZE_2)).unwrap();
    }

    // run cargo bundle
    print_and_wait("\n🎉 Running cargo build...");

    let mut cargo_bundle = "cargo bundle";
    if data.release_build {
        cargo_bundle = "cargo bundle --release";
    }
    util::run_os_command(cargo_bundle, Some(&data.build_dir())).unwrap();

    Ok(())
}

fn build_template(template: String, data: &model::AppData) -> String {
    let mut result = template.to_string();

    result = result.replace("name = \"app_name_lowercased\"", &format!("name = \"{}\"", &data.name.to_lowercase()));
    result = result.replace("name = \"AppName\"", &format!("name = \"{}\"", &data.name));
    result = result.replace("with_url(\"https://www.notion.so\")", &format!("with_url(\"{}\")", &data.url));
    result = result.replace("with_title(\"app_name\")", &format!("with_title(\"{}\")", &data.name));
    result = result.replace("description = \"app_description\"", &format!("description = \"{}\"", &data.description));
    result = result.replace("version = \"0.1.0\"", &format!("version = \"{}\"", &data.version));
    result = result.replace("copyright = \"Copyright © author_name\", ", &format!("copyright = \"Copyright © {}\", ", &data.author));
    result = result.replace("identifier = \"com.example.test\"", &format!("identifier = \"{}\"", &data.identifier));

    result
}

// panics if fails
fn check_pre_requisites() {
    print_and_wait("🎉 Checking prerequisites...");

    // cargo-build
    if !check_executable_exists("cargo-bundle") {
        abort_err(
            "cargo-bundle is not installed.\nPlease install it with `cargo install cargo-bundle`.",
        );
    }

    // convert
    if !check_executable_exists("convert") {
        abort_err("convert is not installed.\nPlease install it with `brew install imagemagick` [macOS]\nCheck readme for other Operating System.");
    }

    print_and_wait("✅ Checking prerequisites done.\n");
}

fn print_and_wait(text: &str) {
    println!("{}", text);
    sleep(Duration::from_millis(SLEEP_TIME));
}

fn check_executable_exists(executable: &str) -> bool {
    util::run_os_command(format!("which {}", executable).as_str(), None).is_ok()
}

fn abort_err(text: &str) {
    println!("Error: \x1b[31m{}\x1b[0m", text);
    exit(1);
}
