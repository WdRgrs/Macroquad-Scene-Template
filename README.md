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
# Pro-tip: definitely generate a cooler name than uncool-name, if possible
cargo generate \
  --git https://github.com/WdRgrs/Macroquad-Scene-Template.git \
  --name uncool-name

cd uncool-name

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

* Verify the WASM build & script
```sh
  cd ./web

  # Start a local server and check it out at: http://localhost:8000/
  python3 -m http.server 8000

```

* Alternatively, or maybe on top of testing the /web directory, it's worth extracting the new .zip file to a new location and testing in a similar fashion (spin up a local server and verify things are running as anticipated)

* Then upload web_{{project-name}}.zip as embedded app 

|> Anticipate assets being a bottleneck with filepath funkyness - play around: copy to different directories, remove asset directories altogether (flatten asset directory tree to root) etc

## Contributing

Contributions are welcome! Feel free to open issues or pull requests to improve this template.


## How to (Scenes)
Scenes are managed through src/scene/manager: 
- startup, update, draw, and cleanup are expected traits, and of course scenes can extend past these.
- Transitions happen when a value is assigned to next_scene OR the update function returns an instance of the GameScene Enum. I find it most convenient to conditionally set the value of next_scene when I'm ready for transitions.
- Added a basic script for quickly building and deploying to itch - will probably add more scripts for quickly deploying to other platofrms in the future (Desktop, then possibly Mobile).
