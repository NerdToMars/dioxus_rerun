#!/bin/bash

echo "🚀 Building Dioxus Rerun Project..."

# Install dependencies
cargo install dioxus-cli

# dx serve

dx serve --release --platform web

# bundle
dx bundle --release --platform web