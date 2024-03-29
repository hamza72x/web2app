### Intro

Convert any web page into a desktop app (but, lightweight ~3MB)

-   The bundle size will be around 3MB
-   Demo: [https://i.imgur.com/BLr03oF.mp4](https://i.imgur.com/BLr03oF.mp4)

### Pre-requisites

#### macOS

```bash
# cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Linux

You should first install the Rust toolchain
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then you need to install some dependencies, which depends on your Linux distribution.

<details>
<summary>Ubuntu (and derivates such as KDE Neon, PopOS etc.)</summary>

```sh
sudo apt install libwebkit2gtk-4.0-dev -y
```

</details>

<details>
<summary>OpenSUSE</summary>

```sh
sudo zypper in -y webkit2gtk3-soup2-devel
```

</details>


<details>
<summary>Other</summary>

You need to look for another package that provides the development libraries for webkit with GTK4.
There might also be other missing packages.

</details>


#### Windows

Windows Setup (x64)

-   Setup Tauri Pre-Requisities for windows: [https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-windows](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-windows)

### Install

```bash
cargo install web2app tauri-cli
```

### Usages

```bash
# Run with args

web2app args -n Notion -u https://www.notion.so -i icon.png
```

```bash
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

-   The output will be saved in `$HOME/web2app_apps/<name>` directory.

### Roadmap

-   [x] Zoom In/Out
-   [x] Reload
-   [x] Standard Copy/Paste
-   [x] Custom User Agent
-   [x] Desktop Notification (working for some apps)
-   [x] Back and forward navigation
-   [ ] Automatically fetch icon from website
-   [ ] Dark Reader Extension (already there but, not fully working)
-   [ ] Retain window size and position after restart
-   [ ] Separate `web2app_apps` repo to host common built apps through Github Action
-   [ ] File download support

### Libraries

-   [Tauri](https://github.com/tauri-apps/tauri)
-   [home](https://github.com/brson/home)
