#!/bin/bash

# Check and remove existing web dir - Probably unnecessary - fresh start
if [ -d "./web" ]; then
  echo "Removing old web dir..."
  rm -rf ./web
fi
mkdir ./web

# Install WebAssembly
echo "Ensure WASM installed..."
rustup target add wasm32-unknown-unknown

# Build WASM
echo "Building WebAssembly binary..."
cargo build --release --target wasm32-unknown-unknown

# Copy the wasm file to the root directory
echo "Copying WASM file to root..."
cp target/wasm32-unknown-unknown/release/{{project-name}}.wasm ./web

# at this time, itch.io is not playing nice with sub-directories under 'assets' - assets work best at root =(
echo "Copying assets file to root..."
if [ -d "assets/sounds" ]; then
  echo "Found sounds"
  cp -r assets/sounds/* ./web
elif [ -d "assets/fonts" ]; then
  echo "Found fonts"
  cp -r assets/fonts/* ./web
fi

echo "Copying index.html file to root..."
cp index.html ./web

echo "Zippity zop"
if [[ "$OSTYPE" == darwin* ]]; then
  echo "Mac-ing"
  zip -r "web_{{project-name}}.zip" web/*
elif [[ "$OSTYPE" == msys* ]]; then
  echo "Win-dos"
  powershell Compress-Archive -Path "web\*" -DestinationPath "web_{{project-name}}.zip" -Force
fi