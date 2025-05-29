# Macroquad Scene Template

A simple scene-management template to quickly spin up a new Macroquad project. It scaffolds a directory structure, includes handy build & deploy scripts, and supports easy customization via **cargo-generate**.

## Features

* Scene & state management boilerplate
* Predefined but flexible directories: `scenes/`, `assets/`, `ui/`
* Starting build & deployment scripts for WebAssembly & itch.io
* Easy WASM packaging built at web_{{project-name}}.zip

## Prerequisites

* Rust & Cargo installed ([https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))
* [cargo-generate](https://github.com/cargo-generate/cargo-generate) installed:

  ```sh
  cargo install cargo-generate
  ```

## Getting Started

Generate a new project from this template:

```sh
cargo generate \
  --git https://github.com/WdRgrs/Macroquad-Scene-Template.git \
  --name cool-name

cd cool-name

cargo run
```

## Directory Structure

```
my-macroquad-game/
├── assets/    # Asset storage, asset handling should be managed in src/
│   ├── fonts/
│   └── sounds/
├── scripts/   # More platforms to comes (WIP)
│   └── itch.sh
├── src/
│   ├── scenes/
│   │   ├── manager.rs
│   │   ├── mod.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   └── button.rs
│   │── app.rs
│   └── main.rs
├── target/
├── web/
│   └── ... # WASM & web assets as an easy .zip target
├── Cargo.lock
├── Cargo.toml
├── .gitignore
└── index.html
```

## Build & Deploy

* **Build WASM & deploy to itch.io**:

  ```sh
  sh scripts/itch.sh

  ```
* Then upload web_{{project-name}}.zip as embedded app 

## Contributing

Contributions are welcome! Feel free to open issues or pull requests to improve this template.
