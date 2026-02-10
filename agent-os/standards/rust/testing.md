# Testing Patterns

## Test Location

**Unit tests**: Colocate in same file with `#[cfg(test)] mod tests`
**Integration tests**: Separate `tests/` directory

```rust
// src/lib.rs
pub struct Foo { ... }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_does_something() { ... }
}
```

Follows standard Rust conventions.

## Test Naming

Descriptive snake_case:
```rust
#[test]
fn layer_state_initializes()  // Good: describes what it tests

#[test]
fn test_1()  // Bad: not descriptive
```

## Testing Global State

Use lock/modify/drop/cleanup pattern:
```rust
#[test]
fn layer_state_can_be_set_globally() {
    // Modify
    {
        let mut state = LAYER_STATE.lock().unwrap();
        *state = Some(LayerState { ... });
    }
    // Verify
    {
        let state = LAYER_STATE.lock().unwrap();
        assert!(state.is_some());
    }
    // Cleanup
    {
        let mut state = LAYER_STATE.lock().unwrap();
        *state = None;
    }
}
```

**Why cleanup?**
- Test isolation (no cross-test pollution)
- Prevent flaky tests (order-dependent failures)
- Verify correct cleanup behavior

## Test Progression

Simple → complex:
1. Test struct defaults
2. Test struct behavior
3. Test global state access
4. Test global state modifications
