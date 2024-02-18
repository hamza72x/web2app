use std::fs;
use std::io;
use std::thread::sleep;
use std::time::Duration;

mod cli;
mod input;
mod model;
mod template_builder;
mod util;
use model::Args;

// milliseconds
const SLEEP_TIME: u64 = 10;

fn main() -> io::Result<()> {
    let mut args = cli::get_args().expect("failed to get cli args");

    // post args processing
    args.update_default_identifier();

    // print given input
    args.print();
    sleep(Duration::from_millis(SLEEP_TIME));

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

    // $HOME/web2app_apps/app_name/icons
    util::re_create_dir(format!("{}/icons", &args.build_dir()).as_str())?;

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

    if let Some(icon) = args.icon.as_ref() {
        // copy icon to web2apps/app_name/icons/app-icon.png
        util::copy_file(&icon, args.app_icon_path()).unwrap();

        // run `cargo tauri icon`
        util::run_os_command_standard(
            format!("cargo tauri icon {}", args.app_icon_path()).as_str(),
            Some(&args.build_dir()),
        )
        .expect("failed to run cargo tauri icon");
    }

    // run cargo tauri build
    print_and_wait("\n🎉 Running cargo tauri build...");

    let mut tauri_build = "cargo tauri build";
    if !args.is_release_build {
        tauri_build = "cargo tauri build --debug";
    }

    util::run_os_command_standard(tauri_build, Some(&args.build_dir())).unwrap();

    Ok(())
}

fn print_and_wait(text: &str) {
    println!("{}", text);
    sleep(Duration::from_millis(SLEEP_TIME));
}
