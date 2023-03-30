use std::collections::HashMap;

use super::consts;
use super::generated;
use super::model::Args;
use super::model::FileBuildData;

pub fn build_template_files(args: &Args) -> [FileBuildData; 11] {
    let files = [
        // template/src/app_config.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/app_config.rs"),
            data_b64: generated::FILE_APP_CONFIG_RS,
            search_replace_texts: None,
        },
        // template/src/app_menu.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/app_menu.rs"),
            data_b64: generated::FILE_APP_MENU_RS,
            search_replace_texts: None,
        },
        // template/src/app_data.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/app_data.rs"),
            data_b64: generated::FILE_APP_DATA_RS,
            search_replace_texts: {
                let mut map = std::collections::HashMap::new();
                if let Some(user_agent) = &args.user_agent {
                    map.insert(
                        String::from("let user_agent: Option<&str> = None;"),
                        format!("let user_agent: Option<&str> = Some(\"{}\");", user_agent),
                    );
                }
                map.insert(
                    String::from("pub const APP_NAME: &str = \"app_name\";"),
                    format!("pub const APP_NAME: &str = \"{}\";", &args.name),
                );
                map.insert(
                    String::from("pub const URL: &str = \"https://www.notion.so/\";"),
                    format!("pub const URL: &str = \"{}\";", &args.url),
                );
                Some(map)
            },
        },
        // template/src/builder.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/builder.rs"),
            data_b64: generated::FILE_BUILDER_RS,
            search_replace_texts: None,
        },
        // template/src/generated.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/generated.rs"),
            data_b64: generated::FILE_GENERATED_RS,
            search_replace_texts: None,
        },
        // template/src/main.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/main.rs"),
            data_b64: generated::FILE_MAIN_RS,
            search_replace_texts: None,
        },
        // template/src/util.rs
        FileBuildData {
            file: args.dest_tmpl_file("src/util.rs"),
            data_b64: generated::FILE_UTIL_RS,
            search_replace_texts: None,
        },
        // template/build.rs
        FileBuildData {
            file: args.dest_tmpl_file("build.rs"),
            data_b64: generated::FILE_BUILD_RS,
            search_replace_texts: None,
        },
        // template/Cargo.toml
        FileBuildData {
            file: args.dest_tmpl_file("Cargo.toml"),
            data_b64: generated::FILE_CARGO_TOML,
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
            data_b64: generated::FILE_CARGO_LOCK,
            search_replace_texts: {
                let mut map = std::collections::HashMap::new();
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
            data_b64: generated::FILE_TAURI_CONF_JSON,
            search_replace_texts: {
                let mut map = std::collections::HashMap::new();
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
                        format!("\"icon\": [{}]", consts::TAURI_ICON_ELEMENTS),
                    );
                }
                Some(map)
            },
        },
    ];

    return files;
}
