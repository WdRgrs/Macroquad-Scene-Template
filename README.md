# Macroquad Scene Template

A simple scene & state-management template to quickly spin up a new Macroquad project. It scaffolds a directory structure, includes handy build & deploy scripts for itch.io, and supports easy customization via **cargo-generate**.

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
  --name my-macroquad-game
cd my-macroquad-game
cargo run
```

## Directory Structure

```
my-macroquad-game/
├── assets/
│   ├── fonts/
│   └── sounds/
├── scripts/
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
│   └── ... # WASM & web assets
├── Cargo.lock
├── Cargo.toml
├── .gitignore
└── index.html
```

## Build & Deploy

* **Build WASM & deploy to itch.io**:

  ```sh
  sh scripts/itch.sh

  <!-- Then upload web_{{project-name}}.zip as embedded app -->
  ```

## Contributing

Contributions are welcome! Feel free to open issues or pull requests to improve this template.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
