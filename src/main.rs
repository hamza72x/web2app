use std::fs;
use std::io;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

mod cli;
mod generated;
mod input;
mod model;
mod template_builder;
mod util;

use model::Args;

// milliseconds
const SLEEP_TIME: u64 = 10;
const ICON_SIZE_1: u8 = 32;
const ICON_SIZE_2: u8 = 128;

fn main() -> io::Result<()> {
    let mut args = cli::get_args().expect("failed to get cli args");

    // post args processing
    args.update_default_identifier();

    // print given input
    args.print();
    sleep(Duration::from_millis(SLEEP_TIME));

    // panics if fails
    check_pre_requisites();

    // building
    build(&args).expect("failed to build");
    sleep(Duration::from_millis(SLEEP_TIME));

    // opening output directory in file explorer
    util::open_dir_in_explorer(&args.bundle_dir());

    Ok(())
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

// TODO:- update for windows
fn check_executable_exists(executable: &str) -> bool {
    util::get_os_exec_out(format!("which {}", executable).as_str(), None).is_ok()
}

fn abort_err(text: &str) {
    println!("Error: \x1b[31m{}\x1b[0m", text);
    exit(1);
}
