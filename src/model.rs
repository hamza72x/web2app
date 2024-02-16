use clap::{Parser, Subcommand};
use std::collections::HashMap;
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
    Args(Args),

    #[command(about = "Builds the app with interactive input.")]
    Interactive,
}

#[derive(Parser)]
pub struct Args {
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
    pub is_release_build: bool,

    #[arg(short = 'g', long, help = "Custom user agent")]
    pub user_agent: Option<String>,
}

// methods
impl Args {
    pub fn update_default_identifier(&mut self) {
        // if identifier is not given, generate from args.url
        if self.identifier == "com.example.testapp" {
            self.identifier = get_identifier_from_url(&self.url);
        }
    }

    // build_dir is the path to the build directory
    // e.g: "$HOME/web2app_apps/app_name"
    pub fn build_dir(&self) -> String {
        let home_dir = home::home_dir().unwrap().display().to_string();

        let path = Path::new(&home_dir);
        let path = path.join("web2app_apps");
        let path = path.join(self.name.to_lowercase());

        path.as_path().display().to_string()
    }

    // icon_path is the path to the icon file in the build directory
    // e.g: for the file "icon.png" the path would be "$HOME/web2app_apps/app_name/icons/app-icon.png"
    pub fn app_icon_path(&self) -> String {
        let build_dir = self.build_dir();
        let path = Path::new(&build_dir);
        let path = path.join("icons").join("app-icon.png");

        path.to_str().unwrap().to_string()
    }

    // bundle_dir is the path to the bundle directory
    // e.g: "$HOME/web2app_apps/app_name/target/debug/bundle"
    pub fn bundle_dir(&self) -> String {
        let build_dir = self.build_dir();
        let path = Path::new(&build_dir);

        let mut build_type = "debug";
        if self.is_release_build {
            build_type = "release";
        }

        let path = path.join("target").join(build_type).join("bundle");
        path.to_str().unwrap().to_string()
    }

    // dest_tmpl_file is the file in the build directory
    // e.g: for the file "src/main.rs" the path would be "$HOME/web2app_apps/app_name/src/main.rs"
    // e.g: for the file "Cargo.toml" the path would be "$HOME/web2app_apps/app_name/Cargo.toml"
    pub fn dest_tmpl_file(&self, path: &str) -> File {
        File::create(self.dest_tmpl_path_buf(path)).unwrap()
    }

    pub fn dest_tmpl_path_buf(&self, path: &str) -> PathBuf {
        PathBuf::from(format!("{}/{}", self.build_dir(), path))
    }

    pub fn print(&self) {
        println!("name: {}", self.name);
        println!("url: {}", self.url);
        println!("description: {}", self.description);
        println!("version: {}", self.version);
        println!("author: {}", self.author);
        println!("identifier: {}", self.identifier);
        println!(
            "icon: {}",
            self.icon.as_ref().unwrap_or(&String::from("None"))
        );
        println!(
            "User Agent: {}",
            self.user_agent.as_ref().unwrap_or(&String::from("Default"))
        );
        println!("");
        println!("🚀 build_dir: {}", self.build_dir());
    }
}

pub struct FileBuildData<'a> {
    pub file: File,
    pub data: &'a str,
    // map if search and replace text
    pub search_replace_texts: Option<HashMap<String, String>>,
}

impl FileBuildData<'_> {
    pub fn decode_and_write(&mut self) {
        let mut data = self.data.to_string();
        if let Some(search_replace_texts) = &self.search_replace_texts {
            for (key, value) in search_replace_texts {
                data = data.replace(key.as_str(), value);
            }
        }
        self.file.write_all(data.as_bytes()).unwrap();
    }
}

fn get_identifier_from_url(url: &String) -> String {
    // remove http(s)://
    let url = url.replace("https://", "");
    let url = url.replace("http://", "");

    // replace all non alphanumeric characters with a dot
    let identifier = url
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' {
                c
            } else {
                '.'
            }
        })
        .collect::<String>();

    let identifier = identifier + ".web2app";

    // replace double dots with a single dot
    return identifier.replace("..", ".");
}
