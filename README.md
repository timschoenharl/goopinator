# ARC Overlay - Proof of Concept

A Vulkan layer overlay for ARC Raiders written in Rust.

## What is this?

This is a proof-of-concept Vulkan layer that demonstrates the ability to inject code into ARC Raiders (or any Vulkan game) running on Linux. The layer intercepts Vulkan API calls and logs when it's active.

## Features (POC)

- ✅ Vulkan layer hook implementation
- ✅ Intercepts `vkQueuePresentKHR` (called every frame)
- ✅ Safe: No memory reading, no file modification
- ✅ Logging to stderr for debugging
- 🚧 Actual overlay rendering (coming next)

## Requirements

- Rust toolchain (installed automatically if needed)
- Vulkan support
- ARC Raiders installed via Steam
- Linux (tested on Nobara/Fedora)

## Installation

1. Run the installation script:
```bash
chmod +x install.sh
./install.sh
```

2. The script will:
   - Build the layer in release mode
   - Install it to `~/.local/share/vulkan/implicit_layer.d/`

## Testing the Layer

### Option 1: Test with vulkaninfo

Check if the layer is recognized:
```bash
vulkaninfo | grep ARC
```

You should see output indicating the layer is loaded.

### Option 2: Test with ARC Raiders

**Important**: Since this is a POC without proper chaining to the next layer, it may cause the game to fail. For testing purposes:

1. Launch Steam from terminal to see stderr output:
```bash
steam
```

2. Configure ARC Raiders to use Vulkan:
   - Right-click ARC Raiders → Properties
   - Launch Options: `-dx12` or just ensure Vulkan is selected in-game

3. Launch the game and check the terminal output for:
```
[ARC Overlay] vkCreateInstance called - Layer is active!
[ARC Overlay] Frame present - Overlay would render here!
```

### Option 3: Safe Testing (Recommended for POC)

Use `vkcube` to test without risking the game:
```bash
vkcube
```

Check terminal output for layer messages.

## Uninstallation

```bash
chmod +x uninstall.sh
./uninstall.sh
```

## Project Structure

```
ARC_Overlay/
├── src/
│   └── lib.rs                  # Vulkan layer implementation
├── Cargo.toml                  # Rust project configuration
├── arc_overlay_layer.json      # Vulkan layer manifest
├── install.sh                  # Installation script
├── uninstall.sh                # Uninstallation script
└── README.md                   # This file
```

## How It Works

1. **Vulkan Layer**: The layer is a shared library (.so) that implements Vulkan layer interfaces
2. **Function Interception**: We intercept key functions like `vkGetInstanceProcAddr`, `vkGetDeviceProcAddr`, and `vkQueuePresentKHR`
3. **Frame Hook**: Every time a frame is about to be presented, our `vkQueuePresentKHR` function is called
4. **Future**: This is where we'll inject ImGui rendering for the actual overlay UI

## Current Limitations (POC)

⚠️ **This is a proof of concept**. Current limitations:

- No actual overlay rendering yet (just logging)
- Chain functions return dummy values (may cause crashes in real games)
- No proper layer chaining implementation
- No error handling for production use
- Logs to stderr (will spam terminal)

## Next Steps

1. **Implement proper layer chaining** to call the next layer/driver
2. **Add ImGui integration** for actual overlay rendering
3. **Implement item scanner** with OCR
4. **Add overlay UI** with item info, maps, etc.
5. **Test with ARC Raiders** once chaining is implemented

## Safety & Anti-Cheat

This overlay is designed to be anti-cheat safe:
- ✅ No memory reading
- ✅ No file modification
- ✅ No gameplay automation
- ✅ Pure rendering overlay

However, always test carefully and be aware that EAC may flag any Vulkan layer as suspicious.

## Building Manually

```bash
cargo build --release
```

The compiled library will be at: `target/release/libarc_overlay.so`

## License

This is a personal project for educational purposes.
