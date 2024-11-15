# Tauri + Vanilla

This template should help get you started developing with Tauri in vanilla HTML, CSS and Javascript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Build

### Development

`cargo tauri dev`

### Production

`cargo tauri build --no-bundle`. 

bundle for distribution outside the macOS App Store 
`cargo tauri bundle --bundles app,dmg`

bundle for App Store distribution
`cargo tauri bundle --bundles app --config src-tauri/tauri.appstore.conf.json`