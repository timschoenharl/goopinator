# Vulkan FFI Safety

## Pointer Null Checks

**Always null-check** before dereferencing:
```rust
#[no_mangle]
pub unsafe extern "C" fn vkGetInstanceProcAddr(
    _instance: vk::Instance,
    p_name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    if p_name.is_null() {
        return None;
    }
    let name = CStr::from_ptr(p_name);
    // ...
}
```

Defensive programming: Never trust external input, even from Vulkan loader.

## String Handling

Use `CStr::from_ptr` for C strings:
```rust
let name = CStr::from_ptr(p_name);
match name.to_bytes() {
    b"vkGetInstanceProcAddr" => { ... }
    _ => {}
}
```

## Function Pointers

Use `as` cast for function pointers:
```rust
// Preferred
return Some(vkGetInstanceProcAddr as vk::PFN_vkVoidFunction);

// Avoid (unnecessary complexity)
return Some(std::mem::transmute(vkGetInstanceProcAddr as *const ()));
```

## Function Naming

**Exact Vulkan spec names**:
```rust
vkGetInstanceProcAddr  // Correct
get_instance_proc_addr // Wrong - doesn't match C ABI expectation
```

## Extern C ABI

All Vulkan entry points:
```rust
#[no_mangle]
pub unsafe extern "C" fn vkFunctionName(...) { ... }
```

- `#[no_mangle]`: Preserve function name for dynamic linking
- `extern "C"`: Use C calling convention
- `unsafe`: FFI is inherently unsafe
