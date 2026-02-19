#!/bin/bash
set -euo pipefail

# fix home issues
export HOME=/root

# Install Rustup
if ! command -v rustup
then
  echo "Installing Rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -t wasm32-unknown-unknown --profile minimal
  source "$HOME/.cargo/env"
else
  echo "Rustup already installed."
  rustup target add wasm32-unknown-unknown
fi

# Install wasm-pack
if ! command -v wasm-pack
then
  echo "Installing wasm-pack..."
  curl https://drager.github.io/wasm-pack/installer/init.sh -sSf | sh
  echo "wasm-pack installation complete."
else
  echo "wasm-pack already installed."
fi

# Build cgol WASM package
echo "Building cgol WASM package..."
cd cgol
wasm-pack build --release --target web
cd ..

# Install Next.js dependencies with bun
echo "Installing Next.js dependencies with bun..."
bun install

