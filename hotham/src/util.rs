#![allow(dead_code)]

use anyhow::Result;
use legion::{Entity, World};
use std::{ffi::CStr, os::raw::c_char, str::Utf8Error};

use cgmath::{vec3, Deg, Euler, Quaternion};

use crate::{
    add_model_to_world,
    components::Transform,
    gltf_loader::load_models_from_gltf,
    resources::{render_context::create_descriptor_set_layouts, VulkanContext},
};

pub(crate) unsafe fn get_raw_strings(strings: Vec<&str>) -> Vec<*const c_char> {
    strings
        .iter()
        .map(|s| CStr::from_bytes_with_nul_unchecked(s.as_bytes()).as_ptr())
        .collect::<Vec<_>>()
}

pub(crate) unsafe fn parse_raw_strings(raw_strings: &[*const c_char]) -> Vec<&str> {
    raw_strings
        .iter()
        .filter_map(|s| parse_raw_string(*s).ok())
        .collect::<Vec<_>>()
}

pub(crate) unsafe fn parse_raw_string(
    raw_string: *const c_char,
) -> Result<&'static str, Utf8Error> {
    let cstr = CStr::from_ptr(raw_string);
    return cstr.to_str();
}

pub(crate) fn to_euler_degrees(rotation: Quaternion<f32>) -> Euler<Deg<f32>> {
    let euler = Euler::from(rotation);
    let degrees = Euler::new(Deg::from(euler.x), Deg::from(euler.y), Deg::from(euler.z));
    degrees
}

pub fn get_world_with_hands() -> World {
    let vulkan_context = VulkanContext::testing().unwrap();
    let set_layouts = create_descriptor_set_layouts(&vulkan_context).unwrap();

    let data: Vec<(&[u8], &[u8])> = vec![
        (
            include_bytes!("../../hotham-asteroid/assets/left_hand.gltf"),
            include_bytes!("../../hotham-asteroid/assets/left_hand.bin"),
        ),
        (
            include_bytes!("../../hotham-asteroid/assets/right_hand.gltf"),
            include_bytes!("../../hotham-asteroid/assets/right_hand.bin"),
        ),
    ];
    let models = load_models_from_gltf(data, &vulkan_context, set_layouts.mesh_layout).unwrap();

    let mut world = World::default();

    // Add two hands
    let left_hand = add_model_to_world("Left Hand", &models, &mut world, None).unwrap();
    {
        let mut left_hand_entity = world.entry(left_hand).unwrap();
        let transform = left_hand_entity.get_component_mut::<Transform>().unwrap();
        transform.translation = vec3(-0.2, 1.4, 0.0);
    }

    let right_hand = add_model_to_world("Right Hand", &models, &mut world, None).unwrap();
    {
        let mut right_hand_entity = world.entry(right_hand).unwrap();
        let transform = right_hand_entity.get_component_mut::<Transform>().unwrap();
        transform.translation = vec3(0.2, 1.4, 0.0);
    }

    world
}

pub fn entity_to_u64(entity: Entity) -> u64 {
    unsafe { std::mem::transmute(entity) }
}
pub fn u64_to_entity(entity: u64) -> Entity {
    unsafe { std::mem::transmute(entity) }
}

#[cfg(target_os = "android")]
pub(crate) fn get_asset_from_path(path: &str) -> Result<Vec<u8>> {
    use anyhow::anyhow;
    let native_activity = ndk_glue::native_activity();

    let asset_manager = native_activity.asset_manager();
    let path_with_nul = format!("{}\0", path);
    let path = unsafe { CStr::from_bytes_with_nul_unchecked(path_with_nul.as_bytes()) };

    let mut asset = asset_manager
        .open(path)
        .ok_or(anyhow!("Can't open: {:?}", path))?;

    Ok(asset.get_buffer()?.to_vec())
}
