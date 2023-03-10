### Intro

Turn any web page into a desktop app (but, lightweight <1MB)

- The bundle will be less than 1MB
- Demo: [https://i.imgur.com/BLr03oF.mp4](https://i.imgur.com/BLr03oF.mp4)

### Features

- General Keyboard Shortcuts
- Dark Reader Extension (`Menu -> Theme`)
- [Your suggestions are welcomed]

### Requirements

- [cargo](https://www.rust-lang.org/tools/install)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- [imagemagick](https://github.com/imagemagick/imagemagick)

```sh
# macOS
brew install imagemagick

# apt
sudo apt install imagemagick
```

- Linux Specific

```sh
# arch
sudo pacman -S webkit2gtk

# debian
sudo apt install -y webkit2gtk
```

### Install

```sh
cargo install nativefier_tauri
```

### Usages 

```sh
# help
nativefier_tauri -h
# Usage: nativefier_tauri [COMMAND]
# 
# Commands:
#   args         Builds the app with the given arguments.
#   interactive  Builds the app with interactive input.
#   help         Print this message or the help of the given subcommand(s)
# 
# Options:
#   -h, --help     Print help
#   -V, --version  Print version

nativefier_tauri args -h

# Usage: nativefier_tauri args [OPTIONS] --name <NAME> --url <URL>
#
# Options:
#   -n, --name <NAME>                The name of the app
#   -u, --url <URL>                  The URL of the app
#   -d, --description <DESCRIPTION>  The description of the app [default: "An example application."]
#   -v, --version <VERSION>          The version of the app [default: 0.1.0]
#   -a, --author <AUTHOR>            The author of the app [default: "John Doe"]
#   -t, --identifier <IDENTIFIER>    The identifier of the app [default: com.example.testapp]
#   -i, --icon <ICON>                The icon of the app
#   -r, --release-build              The release build of the app
#   -h, --help                       Print help

# Run with args
nativefier_tauri args -n Notion -u https://www.notion.so -i icon.png

# Run with interactive mode
nativefier_tauri interactive

# 🍀 Enter Name (TestApp):
# 🍀 Enter URL (https://trello.com):
# 🍀 Enter Description (An example application.):
# 🍀 Enter Version (0.1.0):
# 🍀 Enter Author (John Doe):
# 🍀 Enter Identifier (com.example.testapp):
# 🍀 Enter Icon (icon_path.png):
```

### Output

- The output will be saved in `$HOME/nativefier_tauri_apps/<name>` directory.

### Libraries

- [Tauri](https://github.com/tauri-apps/tauri)
- [wry](https://github.com/tauri-apps/wry)
- [home](https://github.com/brson/home)
