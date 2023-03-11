use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Builds the app with the given arguments.")]
    Args(AppData),

    #[command(about = "Builds the app with interactive input.")]
    Interactive,
}

#[derive(Parser)]
pub struct AppData {
    #[arg(short, long, help = "The name of the app")]
    pub name: String,

    #[arg(short, long, help = "The URL of the app")]
    pub url: String,

    #[arg(short, long, help = "The description of the app", default_value_t = String::from("An example application."))]
    pub description: String,

    #[arg(short, long, help = "The version of the app", default_value_t = String::from("0.1.0"))]
    pub version: String,

    #[arg(short, long, help = "The author of the app", default_value_t = String::from("John Doe"))]
    pub author: String,

    #[arg(short = 't', long, help = "The identifier of the app", default_value_t = String::from("com.example.testapp"))]
    pub identifier: String,

    #[arg(short = 'i', long, help = "The icon of the app")]
    pub icon: Option<String>,

    #[arg(
        short = 'r',
        long,
        help = "The release build of the app",
        default_value_t = true
    )]
    pub release_build: bool,
}

// methods
impl AppData {
    // build_dir is the path to the build directory
    // e.g: "$HOME/nativefier_tauri_apps/app_name"
    pub fn build_dir(&self) -> String {
        let home_dir = home::home_dir().unwrap().display().to_string();

        let path = Path::new(&home_dir);
        let path = path.join("nativefier_tauri_apps");
        let path = path.join(self.name.to_lowercase());

        path.as_path().display().to_string()
    }

    // icon_path is the path to the icon file in the build directory
    // e.g: for the file "icon.png" the path would be "$HOME/nativefier_tauri_apps/app_name/<size>x<size>.png"
    pub fn icon_path(&self, size: u8) -> String {
        let build_dir = self.build_dir();
        let path = Path::new(&build_dir);
        let path = path.join(format!("{}x{}.png", size, size));

        path.to_str().unwrap().to_string()
    }

    // bundle_dir is the path to the bundle directory
    // e.g: "$HOME/nativefier_tauri_apps/app_name/target/debug/bundle"
    pub fn bundle_dir(&self) -> String {
        let build_dir = self.build_dir();
        let path = Path::new(&build_dir);

        let mut build_type = "debug";
        if self.release_build {
            build_type = "release";
        }

        let path = path.join("target").join(build_type).join("bundle");
        path.to_str().unwrap().to_string()
    }

    // dest_tmpl_file is the file in the build directory
    // e.g: for the file "src/main.rs" the path would be "$HOME/nativefier_tauri_apps/app_name/src/main.rs"
    // e.g: for the file "Cargo.toml" the path would be "$HOME/nativefier_tauri_apps/app_name/Cargo.toml"
    pub fn dest_tmpl_file(&self, path: &str) -> File {
        File::create(self.dest_tmpl_path_buf(path)).unwrap()
    }

    pub fn dest_tmpl_path_buf(&self, path: &str) -> PathBuf {
        PathBuf::from(format!("{}/{}", self.build_dir(), path))
    }

    pub fn print(&self) {
        println!("🚀 name: {}", self.name);
        println!("🚀 url: {}", self.url);
        println!("🚀 description: {}", self.description);
        println!("🚀 version: {}", self.version);
        println!("🚀 author: {}", self.author);
        println!("🚀 identifier: {}", self.identifier);
        println!(
            "🚀 icon: {}",
            self.icon.as_ref().unwrap_or(&String::from("None"))
        );
        println!("");
        println!("🚀 build_dir: {}", self.build_dir());
    }
}

pub struct FileBuildData<'a> {
    pub file: File,
    pub data_b64: &'a str,
}

impl FileBuildData<'_> {

    pub fn decode_and_write(&mut self, app_data: &AppData) {
        let data = base64::decode(&self.data_b64).unwrap();
        let data = String::from_utf8(data).unwrap();
        let data = self.build_template(&data, app_data);

        self.file.write_all(data.as_bytes()).unwrap();
    }

    fn build_template(&self, data: &String, app_data: &AppData) -> String {
        let mut result = data.to_string();

        result = result.replace(
            "name = \"app_name_lowercased\"",
            &format!("name = \"{}\"", &app_data.name.to_lowercase()),
        );
        result = result.replace(
            "name = \"AppName\"",
            &format!("name = \"{}\"", &app_data.name),
        );
        result = result.replace(
            "with_url(\"https://www.notion.so\")",
            &format!("with_url(\"{}\")", &app_data.url),
        );
        result = result.replace(
            "with_title(\"app_name\")",
            &format!("with_title(\"{}\")", &app_data.name),
        );
        result = result.replace(
            "description = \"app_description\"",
            &format!("description = \"{}\"", &app_data.description),
        );
        result = result.replace(
            "join(\"app_name\")",
            &format!("join(\"{}\")", &app_data.name.to_lowercase()),
        );
        result = result.replace(
            "version = \"0.1.0\"",
            &format!("version = \"{}\"", &app_data.version),
        );
        result = result.replace(
            "copyright = \"Copyright © author_name\", ",
            &format!("copyright = \"Copyright © {}\", ", &app_data.author),
        );
        result = result.replace(
            "identifier = \"com.example.test\"",
            &format!("identifier = \"{}\"", &app_data.identifier),
        );

        return result;
    }
}
