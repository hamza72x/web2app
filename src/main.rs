use clap::Parser;

use std::fs;
use std::io;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

mod generated;
mod input;
mod model;
mod template_builder;
mod util;

use model::Args;
use model::Cli;
use model::Commands;

// milliseconds
const SLEEP_TIME: u64 = 10;
const ICON_SIZE_1: u8 = 32;
const ICON_SIZE_2: u8 = 128;

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut args: Args;

    match cli.command {
        Some(Commands::Args(arg_data)) => {
            args = arg_data;
        }
        Some(Commands::Interactive) => {
            args = get_interactive_args();
        }
        None => {
            println!("No command given. Use --help for more information.");
            return Err(io::Error::new(io::ErrorKind::Other, "No command given."));
        }
    }

    // post args processing
    args.update_default_identifier();

    // print given input
    args.print();
    sleep(Duration::from_millis(SLEEP_TIME));

    // panics if fails
    check_pre_requisites();

    // building
    build(&args)?;
    sleep(Duration::from_millis(SLEEP_TIME));

    // opening output directory in file explorer
    util::open_dir_in_explorer(&args.bundle_dir());

    Ok(())
}

fn get_interactive_args() -> Args {
    Args {
        name: input::string_must("Name"),
        url: input::string_must("URL"),
        description: input::string("Description", "An example application."),
        version: input::string("Version", "0.1.0"),
        author: input::string("Author", "hamza72x"),
        identifier: input::string("Identifier", "com.example.testapp"),
        icon: input::optional_string("Icon", "icon_path.png"),
        is_release_build: input::bool("Release build", true),
        user_agent: input::optional_string("User agent", "Mozilla/5.0"),
    }
}

// build the app
fn build(args: &Args) -> io::Result<()> {
    // build directories
    print_and_wait("\n🎉 Building directories...");

    // $HOME/web2app_apps/app_name
    fs::create_dir_all(&args.build_dir())?;

    // $HOME/web2app_apps/app_name/src
    util::re_create_dir(format!("{}/src", &args.build_dir()).as_str())?;

    // create files
    print_and_wait("\n🎉 Creating files...");

    // array of FileBuildData
    print_and_wait("\n🎉 Building templates...");

    let mut files = template_builder::build_template_files(args);

    // write files
    for file in files.iter_mut() {
        file.decode_and_write();
    }

    // build icons
    print_and_wait("\n🎉 Building icons...");

    if args.icon.is_some() {
        let source_icon = args.icon.as_ref().unwrap();
        util::resize_icon(&source_icon, ICON_SIZE_1, args.icon_path(ICON_SIZE_1)).unwrap();
        util::resize_icon(&source_icon, ICON_SIZE_2, args.icon_path(ICON_SIZE_2)).unwrap();
    }

    // run cargo bundle
    print_and_wait("\n🎉 Running cargo build...");

    let mut cargo_bundle = "cargo bundle";
    if args.is_release_build {
        cargo_bundle = "cargo bundle --release";
    }
    util::run_os_command_standard(cargo_bundle, Some(&args.build_dir())).unwrap();

    Ok(())
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
    util::get_os_exec_out(format!("which {}", executable).as_str(), None).is_ok()
}

fn abort_err(text: &str) {
    println!("Error: \x1b[31m{}\x1b[0m", text);
    exit(1);
}
