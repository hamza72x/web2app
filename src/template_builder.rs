use std::collections::HashMap;
use std::fmt::format;

use super::model::Args;
use super::model::FileBuildData;

const FILE_APP_CONFIG_RS: &str = include_str!("../template/src/app_config.rs");

const FILE_APP_DATA_RS: &str = include_str!("../template/src/app_data.rs");

const FILE_APP_MENU_RS: &str = include_str!("../template/src/app_menu.rs");

const FILE_BUILDER_RS: &str = include_str!("../template/src/builder.rs");

const FILE_MAIN_RS: &str = include_str!("../template/src/main.rs");

const FILE_UTIL_RS: &str = include_str!("../template/src/util.rs");

const FILE_BUILD_RS: &str = include_str!("../template/build.rs");

const FILE_CARGO_TOML: &str = include_str!("../template/Cargo.toml");

const FILE_CARGO_LOCK: &str = include_str!("../template/Cargo.lock");

const FILE_TAURI_CONF_JSON: &str = include_str!("../template/tauri.conf.json");
const TAURI_ICON_ELEMENTS: &str = r#""icons/32x32.png", "icons/128x128.png", "icons/128x128@2x.png", "icons/icon.icns", "icons/icon.ico""#;

pub fn build_template_files(args: &Args) -> [FileBuildData; 10] {
    let files = [
        // template/src/app_config.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/app_config.rs"),
            data: FILE_APP_CONFIG_RS,
            search_replace_texts: None,
        },
        // template/src/app_menu.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/app_menu.rs"),
            data: FILE_APP_MENU_RS,
            search_replace_texts: None,
        },
        // template/src/app_data.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/app_data.rs"),
            data: FILE_APP_DATA_RS,
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
            data: FILE_BUILDER_RS,
            search_replace_texts: {
                let mut map = HashMap::new();
                map.insert(
                    String::from("INIT_SCRIPT_INSERTED_HERE"),
                    include_str!("../template/src/script.js").to_string(),
                );
                Some(map)
            },
        },
        // template/src/main.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/main.rs"),
            data: FILE_MAIN_RS,
            search_replace_texts: None,
        },
        // template/src/util.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/util.rs"),
            data: FILE_UTIL_RS,
            search_replace_texts: None,
        },
        // template/build.rs
        FileBuildData {
            file: args.dest_tmpl_file("build.rs"),
            data: FILE_BUILD_RS,
            search_replace_texts: None,
        },
        // template/Cargo.toml
        FileBuildData {
            file: args.dest_tmpl_file("Cargo.toml"),
            data: FILE_CARGO_TOML,
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
            data: FILE_CARGO_LOCK,
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
            data: FILE_TAURI_CONF_JSON,
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
