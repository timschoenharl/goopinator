#!/bin/bash

# Goopinator Installation Script

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
LAYER_DIR="$HOME/.local/share/vulkan/implicit_layer.d"
MANIFEST_NAME="goopinator_layer.json"

echo "=== Goopinator Installation ==="
echo

# Create layer directory if it doesn't exist
if [ ! -d "$LAYER_DIR" ]; then
    echo "Creating Vulkan layer directory: $LAYER_DIR"
    mkdir -p "$LAYER_DIR"
fi

# Build the layer in release mode
echo "Building Goopinator layer..."
cd "$SCRIPT_DIR"
$HOME/.cargo/bin/cargo build --release

# Copy the manifest file
echo "Installing layer manifest..."
cp "$SCRIPT_DIR/$MANIFEST_NAME" "$LAYER_DIR/$MANIFEST_NAME"

echo
echo "=== Installation Complete ==="
echo
echo "The layer is installed at: $LAYER_DIR/$MANIFEST_NAME"
echo "Library path: $SCRIPT_DIR/target/release/libgoopinator.so"
echo
echo "To enable the overlay, launch ARC Raiders with Vulkan (layer loads automatically)"
echo
echo "To test if the layer is active, run:"
echo "  vkcube  # Check terminal for [Goopinator] messages"
echo
echo "To disable the overlay temporarily:"
echo "  export DISABLE_GOOPINATOR=1"
echo
echo "To uninstall: ./uninstall.sh"
