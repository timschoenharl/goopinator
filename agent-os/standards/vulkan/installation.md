# Vulkan Layer Installation

## Install Location

**User-local**: `~/.local/share/vulkan/implicit_layer.d/`

Why:
- No sudo required
- Suitable for single-user production
- Easy to install/uninstall

## Installation Modes

### Development Mode (default)
```bash
./install.sh
```

Manifest references build directory:
```json
"library_path": "/path/to/project/target/release/libgoopinator.so"
```

✅ Fast iteration - `cargo build` updates layer
❌ Tied to source location - can't delete project dir

### Production Mode
```bash
./install.sh --production
```

Copies library to stable location:
```json
"library_path": "/home/user/.local/lib/goopinator/libgoopinator.so"
```

✅ Survives `cargo clean`
✅ Source directory can be deleted
❌ Must reinstall after rebuilding

## Manifest Naming

Pattern: `[project_name]_layer.json`
Example: `goopinator_layer.json`

## Environment Variable Disable

Allow users to disable:
```json
"disable_environment": {
    "DISABLE_GOOPINATOR": "1"
}
```

Pattern: `DISABLE_[PROJECT_NAME_UPPER]`
