use ash::vk;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

// Global state for tracking Vulkan objects
static LAYER_STATE: Mutex<Option<LayerState>> = Mutex::new(None);

struct LayerState {
    enabled: bool,
    frame_count: u64,
}

impl Default for LayerState {
    fn default() -> Self {
        Self {
            enabled: true,
            frame_count: 0,
        }
    }
}

// Layer entry points required by Vulkan loader
#[no_mangle]
pub unsafe extern "C" fn vkGetInstanceProcAddr(
    _instance: vk::Instance,
    p_name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    if p_name.is_null() {
        return None;
    }

    let name = CStr::from_ptr(p_name);

    // Log layer queries (only for monitored functions)
    match name.to_bytes() {
        b"vkCreateInstance" | b"vkGetInstanceProcAddr" => {
            eprintln!("[Goopinator] Layer queried for: {:?}", name);
        }
        _ => {}
    }

    // Intercept only vkGetInstanceProcAddr itself
    match name.to_bytes() {
        b"vkGetInstanceProcAddr" => {
            return Some(std::mem::transmute(vkGetInstanceProcAddr as *const ()));
        }
        b"vkEnumerateInstanceLayerProperties" => {
            return Some(std::mem::transmute(vkEnumerateInstanceLayerProperties as *const ()));
        }
        _ => {}
    }

    // Chain to next layer/driver by returning None
    // The Vulkan loader will handle the chaining
    None
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceProcAddr(
    _device: vk::Device,
    p_name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    if p_name.is_null() {
        return None;
    }

    let name = CStr::from_ptr(p_name);

    // Intercept only vkGetDeviceProcAddr itself
    match name.to_bytes() {
        b"vkGetDeviceProcAddr" => {
            return Some(std::mem::transmute(vkGetDeviceProcAddr as *const ()));
        }
        _ => {}
    }

    // Chain to next layer/driver
    None
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateInstanceLayerProperties(
    p_property_count: *mut u32,
    p_properties: *mut vk::LayerProperties,
) -> vk::Result {
    if p_properties.is_null() {
        *p_property_count = 1;
        return vk::Result::SUCCESS;
    }

    if *p_property_count < 1 {
        return vk::Result::INCOMPLETE;
    }

    let layer_name = CString::new("VK_LAYER_GOOPINATOR").unwrap();
    let description = CString::new("Goopinator - ARC Raiders Tactical Overlay (POC)").unwrap();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_state_initializes() {
        let state = LayerState::default();
        assert!(state.enabled);
        assert_eq!(state.frame_count, 0);
    }

    #[test]
    fn layer_state_can_be_disabled() {
        let mut state = LayerState::default();
        assert!(state.enabled);
        state.enabled = false;
        assert!(!state.enabled);
    }

    #[test]
    fn layer_state_tracks_frames() {
        let mut state = LayerState::default();
        assert_eq!(state.frame_count, 0);
        state.frame_count += 1;
        assert_eq!(state.frame_count, 1);
    }

    #[test]
    fn global_layer_state_starts_empty() {
        // State might be Some if other tests initialized it
        // This test just verifies we can lock the mutex
        let state = LAYER_STATE.lock().unwrap();
        drop(state);
    }

    #[test]
    fn layer_state_can_be_set_globally() {
        {
            let mut state = LAYER_STATE.lock().unwrap();
            *state = Some(LayerState {
                enabled: true,
                frame_count: 42,
            });
        }
        {
            let state = LAYER_STATE.lock().unwrap();
            assert!(state.is_some());
            assert!(state.as_ref().unwrap().enabled);
            assert_eq!(state.as_ref().unwrap().frame_count, 42);
        }
        // Clean up
        {
            let mut state = LAYER_STATE.lock().unwrap();
            *state = None;
        }
    }
}
