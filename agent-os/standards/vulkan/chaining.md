# Vulkan Layer Chaining

## POC Chaining (Phase 0.5 - Current)

Return `None` to chain to next layer:
```rust
#[no_mangle]
pub unsafe extern "C" fn vkGetInstanceProcAddr(
    _instance: vk::Instance,
    p_name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    if p_name.is_null() { return None; }
    let name = CStr::from_ptr(p_name);

    match name.to_bytes() {
        b"vkGetInstanceProcAddr" => {
            return Some(vkGetInstanceProcAddr as vk::PFN_vkVoidFunction);
        }
        _ => {}
    }

    None  // Loader handles chaining
}
```

⚠️ **POC only** - Works for basic layer detection, insufficient for production.

## Production Chaining (Phase 1 - Required)

### 1. Dispatch Table Structure

```rust
use std::collections::HashMap;
use std::sync::Mutex;

type PFN_vkGetInstanceProcAddr = unsafe extern "C" fn(VkInstance, *const c_char) -> PFN_vkVoidFunction;
type PFN_vkCreateDevice = unsafe extern "C" fn(VkPhysicalDevice, *const VkDeviceCreateInfo, *const VkAllocationCallbacks, *mut VkDevice) -> VkResult;
type PFN_vkQueuePresentKHR = unsafe extern "C" fn(VkQueue, *const VkPresentInfoKHR) -> VkResult;

struct InstanceDispatch {
    get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    create_device: PFN_vkCreateDevice,
    destroy_instance: PFN_vkDestroyInstance,
}

struct DeviceDispatch {
    get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    queue_present_khr: PFN_vkQueuePresentKHR,
    destroy_device: PFN_vkDestroyDevice,
}

static INSTANCE_DISPATCH: Mutex<HashMap<VkInstance, InstanceDispatch>> = Mutex::new(HashMap::new());
static DEVICE_DISPATCH: Mutex<HashMap<VkDevice, DeviceDispatch>> = Mutex::new(HashMap::new());
```

### 2. Initialize Dispatch in vkCreateInstance

```rust
#[no_mangle]
pub unsafe extern "C" fn vkCreateInstance(
    create_info: *const VkInstanceCreateInfo,
    allocator: *const VkAllocationCallbacks,
    instance: *mut VkInstance,
) -> VkResult {
    // Parse pNext chain to find VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO
    let mut chain_info = (*create_info).p_next;
    let mut layer_link: Option<&VkLayerInstanceLink> = None;

    while !chain_info.is_null() {
        let header = chain_info as *const VkStructureType;
        if *header == VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO {
            let link_info = chain_info as *const VkLayerInstanceCreateInfo;
            layer_link = Some(&(*link_info).u.p_layer_info);
            break;
        }
        chain_info = (*(chain_info as *const VkBaseStructure)).p_next;
    }

    let link = layer_link.expect("Layer link not found in pNext chain");
    let next_get_proc = link.pfnNextGetInstanceProcAddr;

    // Call next layer's vkCreateInstance
    let next_create_instance: PFN_vkCreateInstance =
        std::mem::transmute(next_get_proc(VK_NULL_HANDLE, b"vkCreateInstance\0".as_ptr() as *const c_char));

    let result = next_create_instance(create_info, allocator, instance);

    if result == VK_SUCCESS {
        // Build dispatch table for this instance
        let dispatch = InstanceDispatch {
            get_instance_proc_addr: next_get_proc,
            create_device: std::mem::transmute(next_get_proc(*instance, b"vkCreateDevice\0".as_ptr() as *const c_char)),
            destroy_instance: std::mem::transmute(next_get_proc(*instance, b"vkDestroyInstance\0".as_ptr() as *const c_char)),
        };

        INSTANCE_DISPATCH.lock().unwrap().insert(*instance, dispatch);
    }

    result
}
```

### 3. Chain Device Functions

```rust
#[no_mangle]
pub unsafe extern "C" fn vkQueuePresentKHR(
    queue: VkQueue,
    present_info: *const VkPresentInfoKHR,
) -> VkResult {
    // Our overlay rendering here
    {
        let mut state = LAYER_STATE.lock().unwrap();
        if let Some(s) = state.as_mut() {
            s.frame_count += 1;
            if s.frame_count % 300 == 0 {
                eprintln!("[Goopinator][Frame] {}", s.frame_count);
            }
        }
    }

    // Chain to next layer
    let dispatch = DEVICE_DISPATCH.lock().unwrap();
    let device_dispatch = dispatch.get(&device_from_queue(queue)).expect("Device not found");
    (device_dispatch.queue_present_khr)(queue, present_info)
}
```

**Resources**:
- [Vulkan Loader Docs](https://github.com/KhronosGroup/Vulkan-Loader/blob/main/docs/LoaderInterfaceArchitecture.md)
- [MangoHud dispatch.cpp](https://github.com/flightlessmango/MangoHud) - Production reference

## Selective Interception

**Only intercept what you need** - thousands of calls per frame make logging everything impossible.
