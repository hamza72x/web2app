Create lightweight desktop applications for any website with just one command; but with Rust and Tauri.

- The bundle will be less than 2MB

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
# 🍀 Enter Icon (/Users/user/nativefier_tauri/icon.png):
```

### Output

- The output will be saved in `$HOME/nativefier_tauri/<name>` directory.

### Libraries

- [Tauri](https://github.com/tauri-apps/tauri)
- [wry](https://github.com/tauri-apps/wry)
- [home](https://github.com/brson/home)
