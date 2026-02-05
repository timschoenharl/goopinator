#!/bin/bash

# Goopinator Uninstallation Script

LAYER_DIR="$HOME/.local/share/vulkan/implicit_layer.d"
MANIFEST_NAME="goopinator_layer.json"

echo "=== Goopinator Uninstallation ==="
echo

if [ -f "$LAYER_DIR/$MANIFEST_NAME" ]; then
    echo "Removing layer manifest..."
    rm "$LAYER_DIR/$MANIFEST_NAME"
    echo "Uninstallation complete!"
else
    echo "Layer manifest not found. Nothing to uninstall."
fi

echo
echo "Goopinator has been removed. ARC Raiders will now run without the overlay."
