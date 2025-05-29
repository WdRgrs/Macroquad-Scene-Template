#!/bin/bash

# TODO Ensure project-name works - fallback to swap to
# dir_name=$(basename "$PWD")

# Check and remove existing web dir - Probably unnecessary - fresh start
if [ -d "./web" ]; then
  echo "Removing old web dir..."
  rm -rf ./web/*
fi

echo "$dir_name"
# Install WebAssembly
echo "Ensure WASM installed..."
rustup target add wasm32-unknown-unknown

# Build WASM
echo "Building WebAssembly binary..."
cargo build --release --target wasm32-unknown-unknown


# Copy the wasm file to the root directory
echo "Copying WASM file to root..."
mkdir ./web
# TODO Ensure project-name works - fallback to swap to
cp target/wasm32-unknown-unknown/release/{{project-name}}.wasm ./web
# cp target/wasm32-unknown-unknown/release/"$dir_name".wasm ./web

# at this time, itch.io is not playing nice with sub-directories under 'assets' - assets work best at root =(
echo "Copying assets file to root..."
cp -r assets/sounds/* ./web
cp -r assets/fonts/* ./web

echo "Copying index.html file to root..."
cp index.html ./web


echo "Zippity zop"
if [[ "$OSTYPE" == darwin* ]]; then
  echo "Mac-ing"
  # TODO Ensure project-name works - fallback to swap to
  zip -r "web_{{project-name}}.zip" web/*
fi

if [[ "$OSTYPE" == msys* ]]; then
  echo "Win-dos"
  # TODO Ensure project-name works - fallback to swap to
  powershell Compress-Archive -Path "web\*" -DestinationPath "web_{{project-name}}.zip" -Force

fi
# windows
# mac