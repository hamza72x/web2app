use std::fs;
use std::io;
use std::io::Write;
use std::path;
use std::thread::sleep;
use std::time::Duration;

mod input;
mod model;
mod template;
mod util;

// milliseconds
const SLEEP_TIME: u64 = 500;
const ICON_SIZE_1: u8 = 32;
const ICON_SIZE_2: u8 = 128;

fn main() -> io::Result<()> {
    // panics if fails
    check_pre_requisites();

    let mut data: model::Data = model::Data {
        name: String::from("TestApp"),
        url: String::from("https://trello.com"),
        description: String::from("An example application."),
        version: String::from("0.1.0"),
        author: String::from("John Doe"),
        identifier: String::from("com.example.testapp"),
        icon: None,
        isReleaseBuild: false,
    };

    data.name = input::string("Name", "TestApp");
    data.url = input::string("URL", "https://trello.com");
    data.description = input::string("Description", "An example application.");
    data.version = input::string("Version", "0.1.0");
    data.author = input::string("Author", "John Doe");
    data.identifier = input::string("Identifier", "com.example.testapp");
    data.icon = input::optional_string("Icon", "icon_path.png");
    data.isReleaseBuild = input::bool("Release build", false);

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

// build the app
fn build(data: &model::Data) -> io::Result<()> {
    // build directories
    print_and_wait("\n🎉 Building directories...");

    fs::create_dir_all(&data.build_dir())?;
    util::re_create_dir(&data.src_dir())?;

    // create files
    print_and_wait("\n🎉 Creating files...");

    let mut cargo_toml = fs::File::create(&path::PathBuf::from(&data.cargo_toml_path())).unwrap();
    let mut main_rs = fs::File::create(&path::PathBuf::from(&data.main_rs_path())).unwrap();

    print_and_wait("\n🎉 Writing to files...");

    cargo_toml.write_all(build_template(template::CARGO_TOML, &data).as_bytes()).unwrap();
    main_rs.write_all(build_template(template::MAIN_RS, &data).as_bytes()).unwrap();

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
    if data.isReleaseBuild {
        cargo_bundle = "cargo bundle --release";
    }
    util::run_os_command(cargo_bundle, Some(&data.build_dir())).unwrap();

    Ok(())
}

fn build_template(template: &str, data: &model::Data) -> String {
    let mut result = template.to_string();

    result = result.replace("%name%", &data.name);
    result = result.replace("%name_lower_cased%", &data.name.to_lowercase());
    result = result.replace("%url%", &data.url);
    result = result.replace("%description%", &data.description);
    result = result.replace("%version%", &data.version);
    result = result.replace("%author%", &data.author);
    result = result.replace("%identifier%", &data.identifier);

    result
}

// panics if fails
fn check_pre_requisites() {
    print_and_wait("🎉 Checking prerequisites...");

    // cargo-build
    print_and_wait("\n🎉 Checking if cargo-bundle is installed...");
    util::run_os_command("cargo install cargo-bundle", None).unwrap();

    // convert
    print_and_wait("\n🎉 Checking if convert is installed...");
    util::run_os_command("convert -version", None).unwrap();

    print_and_wait("Checking prerequisites done.\n");
}

fn print_and_wait(text: &str) {
    println!("{}", text);
    sleep(Duration::from_millis(SLEEP_TIME));
}
