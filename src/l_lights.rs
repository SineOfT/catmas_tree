use bevy::{
    prelude::*,
    render::render_resource::{ShaderRef, ShaderType, AsBindGroup}
};


pub const LIGHT_MASK_COUNT: usize = 36;
pub const MAX_LIGHT_ALTER_COUNT: usize = 16;


#[derive(ShaderType, Clone, Copy, Default)]
pub struct LightMaskItem {
    color: Vec3,
    t_offset: f32,
    t_timings: [Vec4; MAX_LIGHT_ALTER_COUNT]
}

impl LightMaskItem {
    pub fn new(color: Vec3, t_offset: f32, t_timings: [Vec4; MAX_LIGHT_ALTER_COUNT]) -> Self {
        Self { color, t_offset, t_timings }
    }
}


#[derive(AsBindGroup, Asset, TypePath, Clone)]
pub struct LLights {
    #[texture(0)]
    #[sampler(1)]
    tree_sprite: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    lights_sprite: Handle<Image>,
    #[texture(4)]
    #[sampler(5)]
    light_mask_sprite: Handle<Image>,
    #[uniform(6)]
    light_mask_items: [LightMaskItem; LIGHT_MASK_COUNT],
    #[uniform(7)]
    duration: f32
}

impl LLights {
    pub fn new(asset_server: &AssetServer, light_mask_items: [LightMaskItem; LIGHT_MASK_COUNT], duration: f32) -> Self {
        Self {
            tree_sprite: asset_server.load("tree.png"),
            lights_sprite: asset_server.load("lights.png"),
            light_mask_sprite: asset_server.load("light_mask.png"),
            light_mask_items,
            duration
        }
    }
}

impl UiMaterial for LLights {
    fn fragment_shader() -> ShaderRef {
        "lights_shader.wgsl".into()
    }
}