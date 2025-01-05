use bevy::{
    prelude::*,
    render::render_resource::{ShaderRef, AsBindGroup}
};


#[derive(AsBindGroup, Asset, TypePath, Clone)]
pub struct LColorSwatch {
    #[uniform(0)]
    hue: f32,
    #[uniform(1)]
    alpha: f32
}

impl LColorSwatch {
    pub fn new() -> Self {
        Self { hue: 0., alpha: 1. }
    }
}

impl UiMaterial for LColorSwatch {
    fn fragment_shader() -> ShaderRef {
        "color_wheel_shader.wgsl".into()
    }
}