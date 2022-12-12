pub struct Data {
    pub name: String,
    pub url: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub identifier: String,
    pub icon: Option<String>,
    pub is_release_build: bool,
}

// methods
impl Data {
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
        if self.is_release_build {
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

