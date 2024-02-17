use std::collections::HashMap;
use super::model::Args;
use super::model::FileBuildData;

extern crate proc_macro;

// Uses absolute path of current working directory instead of relative path because that won't work when publishing
macro_rules! include_template_file {
    ($rel_path:expr) => {
        include_str!(concat!(env!("PWD"), "/template/", $rel_path))
    };
}

const TAURI_ICON_ELEMENTS: &str = r#""icons/32x32.png", "icons/128x128.png", "icons/128x128@2x.png", "icons/icon.icns", "icons/icon.ico""#;

pub fn build_template_files(args: &Args) -> [FileBuildData; 10] {
    let files = [
        // template/src/app_config.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/app_config.rs"),
            data: include_template_file!("src/app_config.rs"),
            search_replace_texts: None,
        },
        // template/src/app_menu.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/app_menu.rs"),
            data: include_template_file!("src/app_menu.rs"),
            search_replace_texts: None,
        },
        // template/src/app_data.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/app_data.rs"),
            data: include_template_file!("src/app_data.rs"),
            search_replace_texts: {
                let mut map = HashMap::new();
                if let Some(user_agent) = &args.user_agent {
                    map.insert(
                        String::from("pub const USER_AGENT: Option<&str> = None;"),
                        format!("pub const USER_AGENT: Option<&str> = Some(\"{}\");", user_agent),
                    );
                }
                map.insert(
                    String::from("pub const APP_NAME: &str = \"app_name\";"),
                    format!("pub const APP_NAME: &str = \"{}\";", &args.name),
                );
                map.insert(
                    String::from("pub const URL: &str = \"https://google.com\";"),
                    format!("pub const URL: &str = \"{}\";", &args.url),
                );
                Some(map)
            },
        },
        // template/src/builder.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/builder.rs"),
            data: include_template_file!("src/builder.rs"),
            search_replace_texts: {
                let mut map = HashMap::new();
                map.insert(
                    String::from("INIT_SCRIPT_INSERTED_HERE"),
                    include_template_file!("src/script.js").to_string(),
                );
                Some(map)
            },
        },
        // template/src/main.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/main.rs"),
            data: include_template_file!("src/main.rs"),
            search_replace_texts: None,
        },
        // template/src/util.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/util.rs"),
            data: include_template_file!("src/util.rs"),
            search_replace_texts: None,
        },
        // template/build.rs
        FileBuildData {
            file: args.dest_tmpl_file("build.rs"),
            data: include_template_file!("build.rs"),
            search_replace_texts: None,
        },
        // template/Cargo.toml
        FileBuildData {
            file: args.dest_tmpl_file("Cargo.toml"),
            data: include_template_file!("Cargo.toml"),
            search_replace_texts: {
                let mut map = HashMap::new();
                map.insert(
                    String::from("name = \"app_name_lowercased\""),
                    format!("name = \"{}\"", &args.name.to_lowercase()),
                );
                map.insert(
                    String::from("description = \"test_description\""),
                    format!("description = \"{}\"", &args.description),
                );
                map.insert(
                    String::from("version = \"0.1.0\""),
                    format!("version = \"{}\"", &args.version),
                );
                Some(map)
            },
        },
        // template/Cargo.lock
        FileBuildData {
            file: args.dest_tmpl_file("Cargo.lock"),
            data: include_template_file!("Cargo.lock"),
            search_replace_texts: {
                let mut map = HashMap::new();
                map.insert(
                    String::from("name = \"app_name_lowercased\""),
                    format!("name = \"{}\"", &args.name.to_lowercase()),
                );
                Some(map)
            },
        },
        // template/tauri.conf.json
        FileBuildData {
            file: args.dest_tmpl_file("tauri.conf.json"),
            data: include_template_file!("tauri.conf.json"),
            search_replace_texts: {
                let mut map = HashMap::new();
                map.insert(
                    String::from("\"productName\": \"app_name\""),
                    format!("\"productName\": \"{}\"", &args.name),
                );
                map.insert(
                    String::from("\"identifier\": \"com.example.test\""),
                    format!("\"identifier\": \"{}\"", &args.identifier),
                );
                map.insert(
                    String::from("\"version\": \"0.1.0\""),
                    format!("\"version\": \"{}\"", &args.version),
                );
                if args.icon.is_some() {
                    map.insert(
                        String::from("\"icon\": []"),
                        format!("\"icon\": [{}]", TAURI_ICON_ELEMENTS),
                    );
                }
                Some(map)
            },
        },
    ];

    return files;
}
