use bevy::{
    prelude::*,
    render::render_resource::{ShaderRef, AsBindGroup}
};


#[derive(AsBindGroup, Asset, TypePath, Clone)]
pub struct LColorSwatch {
    #[uniform(0)]
    hue: f32,
    #[uniform(1)]
    alpha: f32,
    #[uniform(2)]
    whiteness: f32,
    #[uniform(3)]
    blackness: f32
}

impl LColorSwatch {
    pub fn new() -> Self {
        Self { hue: 0., alpha: 1., whiteness: 0., blackness: 0. }
    }

    pub fn set_hue(&mut self, hue: f32) { self.hue = hue; }
    pub fn set_alpha(&mut self, alpha: f32) { self.alpha = alpha; }
    pub fn set_whiteness(&mut self, whiteness: f32) { self.whiteness = whiteness; }
    pub fn set_blackness(&mut self, blackness: f32) { self.blackness = blackness; }
}

impl UiMaterial for LColorSwatch {
    fn fragment_shader() -> ShaderRef {
        "color_swatch_shader.wgsl".into()
    }
}