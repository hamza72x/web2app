use clap::{Parser, Subcommand};

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

    #[arg(short = 'r', long, help = "The release build of the app", default_value_t = true)]
    pub release_build: bool,
}

// methods
impl AppData {
    pub fn build_dir(&self) -> String {
        let home_dir = home::home_dir().unwrap().display().to_string();

        let path = std::path::Path::new(&home_dir);
        let path = path.join("nativefier_tauri_apps");
        let path = path.join(self.name.to_lowercase());

        path.as_path().display().to_string()
    }

    pub fn icon_path(&self, size: u8) -> String {
        let build_dir = self.build_dir();
        let path = std::path::Path::new(&build_dir);
        let path = path.join(format!("{}x{}.png", size, size));

        path.to_str().unwrap().to_string()
    }

    pub fn bundle_dir(&self) -> String {
        let build_dir = self.build_dir();
        let path = std::path::Path::new(&build_dir);

        let mut build_type = "debug";
        if self.release_build {
            build_type = "release";
        }

        let path = path.join("target").join(build_type).join("bundle");
        path.to_str().unwrap().to_string()
    }

    pub fn src_dir(&self) -> String {
        format!("{}/src", self.build_dir())
    }

    pub fn cargo_toml_path(&self) -> String {
        format!("{}/Cargo.toml", self.build_dir())
    }

    pub fn main_rs_path(&self) -> String {
        format!("{}/main.rs", self.src_dir())
    }

    pub fn print(&self) {
        println!("🚀 name: {}", self.name);
        println!("🚀 url: {}", self.url);
        println!("🚀 description: {}", self.description);
        println!("🚀 version: {}", self.version);
        println!("🚀 author: {}", self.author);
        println!("🚀 identifier: {}", self.identifier);
        println!("🚀 icon: {:?}", self.icon);
        println!("");
        println!("🚀 build_dir: {}", self.build_dir());
    }
}
