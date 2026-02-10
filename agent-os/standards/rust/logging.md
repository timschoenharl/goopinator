# Logging

Use `eprintln!` for debug logging (outputs to stderr).

## Why eprintln!

- Minimal dependencies - keep the layer lightweight
- Zero overhead when stderr is redirected
- Standard Vulkan layer debugging approach

## Format

Structured with context:
```rust
eprintln!("[Goopinator][Layer] vkGetInstanceProcAddr called");
eprintln!("[Goopinator][State] Frame count: {}", count);
eprintln!("[Goopinator][Error] Failed to initialize: {}", err);
```

Pattern: `[ProjectName][Component] message`

## Production

Keep all debug logs in production. Users can disable:
```bash
command 2>/dev/null  # Silence stderr
export DISABLE_GOOPINATOR=1  # Disable layer entirely
```

## When to Log

- Layer initialization/shutdown
- Important state changes
- Errors and warnings
- Selectively in hot paths (not every frame)
