# NQUI
## Lightweight, performance database explorer


nqui is a lightweight, performant desktop app, created with Tauri, for exploring nqlite databases.

## Features

- Connect to an nqlite database and explore without having to run curl requests
- Run realtime queries
- Extremely performant due to Rust

## Getting Started

Open your terminal and run these commands.

```sh
npm install
npm run tauri dev
```
For Linux Users

You may have some dependency issues when running the build.
For instance, I had to run 
```sh
sudo apt update && sudo apt install libwebkit2gtk-4.0-dev
```
for the build to be successful
