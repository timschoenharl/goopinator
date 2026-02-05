use ash::vk;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

// Global state for tracking Vulkan objects
static LAYER_STATE: Mutex<Option<LayerState>> = Mutex::new(None);

struct LayerState {
    enabled: bool,
}

impl Default for LayerState {
    fn default() -> Self {
        Self { enabled: true }
    }
}

// Layer entry points required by Vulkan loader
#[no_mangle]
pub unsafe extern "C" fn vkGetInstanceProcAddr(
    instance: vk::Instance,
    p_name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    if p_name.is_null() {
        return None;
    }

    let name = CStr::from_ptr(p_name);

    // Log that our layer is being queried
    eprintln!("[Goopinator] vkGetInstanceProcAddr called for: {:?}", name);

    match name.to_bytes() {
        b"vkGetInstanceProcAddr" => {
            Some(std::mem::transmute(vkGetInstanceProcAddr as *const ()))
        }
        b"vkCreateInstance" => {
            Some(std::mem::transmute(vkCreateInstance as *const ()))
        }
        b"vkDestroyInstance" => {
            Some(std::mem::transmute(vkDestroyInstance as *const ()))
        }
        b"vkEnumerateInstanceLayerProperties" => {
            Some(std::mem::transmute(vkEnumerateInstanceLayerProperties as *const ()))
        }
        _ => {
            // Chain to the next layer or driver
            get_instance_proc_addr_chain(instance, p_name)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceProcAddr(
    device: vk::Device,
    p_name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    if p_name.is_null() {
        return None;
    }

    let name = CStr::from_ptr(p_name);

    eprintln!("[Goopinator] vkGetDeviceProcAddr called for: {:?}", name);

    match name.to_bytes() {
        b"vkGetDeviceProcAddr" => {
            Some(std::mem::transmute(vkGetDeviceProcAddr as *const ()))
        }
        b"vkQueuePresentKHR" => {
            // This is where we'll inject our overlay rendering
            Some(std::mem::transmute(vkQueuePresentKHR as *const ()))
        }
        _ => {
            // Chain to the next layer or driver
            get_device_proc_addr_chain(device, p_name)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateInstance(
    p_create_info: *const vk::InstanceCreateInfo,
    p_allocator: *const vk::AllocationCallbacks,
    p_instance: *mut vk::Instance,
) -> vk::Result {
    eprintln!("[Goopinator] vkCreateInstance called - Layer is active!");

    // Initialize our layer state
    *LAYER_STATE.lock().unwrap() = Some(LayerState::default());

    // Chain to the actual create instance
    create_instance_chain(p_create_info, p_allocator, p_instance)
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyInstance(
    instance: vk::Instance,
    p_allocator: *const vk::AllocationCallbacks,
) {
    eprintln!("[Goopinator] vkDestroyInstance called - Cleaning up");

    // Clear our layer state
    *LAYER_STATE.lock().unwrap() = None;

    // Chain to the actual destroy instance
    destroy_instance_chain(instance, p_allocator)
}

#[no_mangle]
pub unsafe extern "C" fn vkQueuePresentKHR(
    queue: vk::Queue,
    p_present_info: *const vk::PresentInfoKHR,
) -> vk::Result {
    // This is called before every frame is presented
    // Perfect place to inject overlay rendering

    if let Some(state) = LAYER_STATE.lock().unwrap().as_ref() {
        if state.enabled {
            eprintln!("[Goopinator] Frame present - Overlay would render here!");
            // TODO: Actual overlay rendering will go here
        }
    }

    // Chain to the actual present
    queue_present_chain(queue, p_present_info)
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateInstanceLayerProperties(
    p_property_count: *mut u32,
    p_properties: *mut vk::LayerProperties,
) -> vk::Result {
    eprintln!("[Goopinator] vkEnumerateInstanceLayerProperties called");

    if p_properties.is_null() {
        *p_property_count = 1;
        return vk::Result::SUCCESS;
    }

    if *p_property_count < 1 {
        return vk::Result::INCOMPLETE;
    }

    let layer_name = CString::new("VK_LAYER_GOOPINATOR").unwrap();
    let description = CString::new("Goopinator - ARC Raiders Tactical Overlay").unwrap();

    let mut properties = vk::LayerProperties::default();
    std::ptr::copy_nonoverlapping(
        layer_name.as_ptr(),
        properties.layer_name.as_mut_ptr() as *mut c_char,
        layer_name.as_bytes_with_nul().len().min(256),
    );
    std::ptr::copy_nonoverlapping(
        description.as_ptr(),
        properties.description.as_mut_ptr() as *mut c_char,
        description.as_bytes_with_nul().len().min(256),
    );
    properties.spec_version = vk::make_api_version(0, 1, 3, 0);
    properties.implementation_version = 1;

    *p_properties = properties;
    *p_property_count = 1;

    vk::Result::SUCCESS
}

// Chain functions - these will call the next layer or driver
// In a real implementation, we'd need to store the function pointers from the chain
unsafe fn get_instance_proc_addr_chain(
    _instance: vk::Instance,
    _p_name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    // For now, return None to let Vulkan loader handle it
    None
}

unsafe fn get_device_proc_addr_chain(
    _device: vk::Device,
    _p_name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    None
}

unsafe fn create_instance_chain(
    _p_create_info: *const vk::InstanceCreateInfo,
    _p_allocator: *const vk::AllocationCallbacks,
    _p_instance: *mut vk::Instance,
) -> vk::Result {
    // This would normally chain to the next layer
    // For POC, we'll just return success
    vk::Result::SUCCESS
}

unsafe fn destroy_instance_chain(
    _instance: vk::Instance,
    _p_allocator: *const vk::AllocationCallbacks,
) {
    // Chain to next layer
}

unsafe fn queue_present_chain(
    _queue: vk::Queue,
    _p_present_info: *const vk::PresentInfoKHR,
) -> vk::Result {
    // Chain to next layer
    vk::Result::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_state_initializes() {
        let state = LayerState::default();
        assert!(state.enabled);
    }
}
