### Intro

Turn any web page into a desktop app (but, lightweight <1MB)

- The bundle will be less than 1MB
- Demo: [https://i.imgur.com/BLr03oF.mp4](https://i.imgur.com/BLr03oF.mp4)

### Features

- General Keyboard Shortcuts
- Dark Reader Extension (`Menu -> Theme`)
- [Your suggestions are welcomed]

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
# 🍀 Enter Icon (/Users/user/nativefier_tauri_apps/icon.png):
```

### Output

- The output will be saved in `$HOME/nativefier_tauri_apps/<name>` directory.

### Libraries

- [Tauri](https://github.com/tauri-apps/tauri)
- [wry](https://github.com/tauri-apps/wry)
- [home](https://github.com/brson/home)
