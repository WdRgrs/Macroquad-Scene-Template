## Notes

Below are some interesting issues I've run into when porting to various platforms and a handy workaround that I don't want to get lost in the stack

```rs
// May need to add this to main.rs if trouble finding assets in src 
set_pc_assets_folder("assets");

// Have previously had to prepend directories for windows env, but not wasm
let path = "Random-Font.ttf";
let path = if cfg!(target_os = "windows") {
    format!("assets/fonts/{path}")
} else if cfg!(target_arch = "wasm32") {
    path.to_string()
} else {
    path.to_string()
};

load_ttf_font(&path).await
```