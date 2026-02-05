# Quick Start Guide

## Installation (5 minutes)

1. **Install the overlay**:
   ```bash
   cd /home/timothys/Projects/ARC_Overlay
   ./install.sh
   ```

2. **Verify installation**:
   ```bash
   ls ~/.local/share/vulkan/implicit_layer.d/arc_overlay_layer.json
   ```

3. **Test with vkcube**:
   ```bash
   vkcube
   ```

   You should see `[ARC Overlay]` messages in the terminal.

## Testing with ARC Raiders

⚠️ **WARNING**: The current POC doesn't properly chain to the next Vulkan layer, so it may crash ARC Raiders. Proceed with caution.

### Safe Testing Approach:

1. **Launch Steam from terminal** (to see stderr output):
   ```bash
   steam
   ```

2. **Configure ARC Raiders**:
   - Right-click ARC Raiders in Steam → Properties
   - Verify it's using Vulkan (not DirectX 12)
   - The layer will load automatically when using Vulkan

3. **Launch the game** and watch the terminal for:
   ```
   [ARC Overlay] vkCreateInstance called - Layer is active!
   ```

4. **If the game crashes**:
   - Disable the overlay: `./uninstall.sh`
   - Continue development on layer chaining

### Disable the overlay temporarily:

```bash
export DISABLE_ARC_OVERLAY=1
steam  # Overlay won't load
```

### Disable permanently:

```bash
./uninstall.sh
```

## Development Workflow

### Make changes to the code:
```bash
nano src/lib.rs  # or use your preferred editor
```

### Rebuild and reinstall:
```bash
./install.sh
```

### Test:
```bash
vkcube
```

## What to Expect

### ✅ Currently Working:
- Layer loads when any Vulkan application starts
- Intercepts Vulkan function calls
- Logs to stderr
- Can be toggled on/off

### ❌ Not Yet Working:
- Proper layer chaining (may cause crashes)
- Actual overlay rendering (just logging)
- Item scanning
- UI elements

## Next Development Steps

See `POC_STATUS.md` for detailed next steps. The immediate priority is:

1. **Implement layer chaining** so the overlay doesn't crash games
2. **Add ImGui** for actual overlay rendering
3. **Test with ARC Raiders** once stable

## Troubleshooting

### "Layer doesn't load"
Check if manifest exists:
```bash
cat ~/.local/share/vulkan/implicit_layer.d/arc_overlay_layer.json
```

### "vkcube crashes"
The layer chaining isn't implemented yet. This is expected.

### "No [ARC Overlay] messages"
Layer might be disabled. Check:
```bash
echo $DISABLE_ARC_OVERLAY
```

If it prints "1", unset it:
```bash
unset DISABLE_ARC_OVERLAY
```

### "Compilation errors"
Make sure Rust is installed:
```bash
rustc --version
```

## File Structure

```
ARC_Overlay/
├── src/lib.rs              # Main Vulkan layer code
├── Cargo.toml              # Rust project config
├── arc_overlay_layer.json  # Vulkan layer manifest
├── install.sh              # Installation script
├── uninstall.sh            # Uninstallation script
├── README.md               # Full documentation
├── POC_STATUS.md           # Current status
└── QUICKSTART.md           # This file
```

## Getting Help

- Check `README.md` for full documentation
- Check `POC_STATUS.md` for current status and known issues
- Check the Rust code in `src/lib.rs` for implementation details

## Success Indicators

When the layer is working correctly with vkcube:
1. ✅ Terminal shows `[ARC Overlay]` messages
2. ✅ vkcube runs without crashing
3. ✅ No Vulkan validation errors

When ready for ARC Raiders:
1. ✅ Layer chaining implemented
2. ✅ Tested with multiple Vulkan apps
3. ✅ No crashes in test applications
