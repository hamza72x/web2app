### Intro

Convert any web page into a desktop app (but, lightweight ~1MB)

- The bundle size will be around 1MB
- Demo: [https://i.imgur.com/BLr03oF.mp4](https://i.imgur.com/BLr03oF.mp4)

### Pre-requisites

#### macOS

```sh
# cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# imagemagick
brew install imagemagick
```

#### Linux 

- Given example is for Ubuntu, but it should work for other distros as well with respective packages.

```sh
# cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# imagemagick
sudo apt install imagemagick -y

# webkit2gtk
sudo apt install libwebkit2gtk-4.0-dev -y
```

#### Windows

- TODO

### Install

```sh
cargo install web2app tauri-cli
```

### Usages 

```sh
# Run with args

web2app args -n Notion -u https://www.notion.so -i icon.png
```

```sh
# Run with interactive mode

web2app interactive

# 🍀 Enter Name (TestApp):
# 🍀 Enter URL (https://trello.com):
# 🍀 Enter Description (An example application.):
# 🍀 Enter Version (0.1.0):
# 🍀 Enter Author (John Doe):
# 🍀 Enter Identifier (com.example.testapp):
# 🍀 Enter Icon (icon_path.png):
# 🍀 Enter User Agent (Mozilla/5.0):
```

### Output

- The output will be saved in `$HOME/web2app_apps/<name>` directory.

### Roadmap

- [x] Zoom In/Out
- [x] Reload
- [x] Standard Copy/Paste
- [ ] Dark Reader Extension (already there but, not fully working)
- [x] Custom User Agent
- [ ] Desktop Notification (useful for Discord like app)
- [ ] Custom url navigation
- [ ] Github Action to build binary
- [ ] Separate `native_tauri_apps` repo to host common built apps through Github Action
- [ ] Retain window size and position after restart
- [ ] File download support
- [ ] Automatically fetch icon from website
- [ ] Docs in crates.io

### Libraries

- [Tauri](https://github.com/tauri-apps/tauri)
- [wry](https://github.com/tauri-apps/wry)
- [home](https://github.com/brson/home)
