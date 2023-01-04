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

### Run 

```sh
# Run
nativefier_tauri

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
