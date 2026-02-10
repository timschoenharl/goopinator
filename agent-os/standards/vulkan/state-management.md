# Vulkan Layer State Management

## Global State Pattern

Use `static Mutex<Option<T>>`:
```rust
static LAYER_STATE: Mutex<Option<LayerState>> = Mutex::new(None);

struct LayerState {
    enabled: bool,
    frame_count: u64,
}
```

**Why `Option<T>`?**
Lazy initialization - state is `None` until first use, avoiding premature initialization before Vulkan loader calls entry points.

## State Struct with Default

Implement `Default` trait:
```rust
impl Default for LayerState {
    fn default() -> Self {
        Self {
            enabled: true,
            frame_count: 0,
        }
    }
}
```

Standard Rust pattern for types with sensible defaults.

## State Access

Lock, use, drop:
```rust
// Read
{
    let state = LAYER_STATE.lock().unwrap();
    if let Some(s) = state.as_ref() {
        eprintln!("Frame: {}", s.frame_count);
    }
}

// Modify
{
    let mut state = LAYER_STATE.lock().unwrap();
    if state.is_none() {
        *state = Some(LayerState::default());
    }
    state.as_mut().unwrap().frame_count += 1;
}
```

## State Lifecycle

1. **None** - Uninitialized (before first Vulkan call)
2. **Some** - Active (layer initialized)
3. **None** - Cleaned up (after layer shutdown)
