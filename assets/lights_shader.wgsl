#import bevy_ui::ui_vertex_output::UiVertexOutput;
#import bevy_render::globals::Globals;

const LIGHT_MASK_R: u32 = 3;
const LIGHT_MASK_G: u32 = 3;
const LIGHT_MASK_B: u32 = 2;
const LIGHT_MASK_A: u32 = 2;
const LIGHT_MASK_COUNT = LIGHT_MASK_R * LIGHT_MASK_G * LIGHT_MASK_B * LIGHT_MASK_A;

const MAX_LIGHT_ALTER_COUNT = 16;
const EPSILON = 0.000001f;


struct LightMaskItem {
    color: vec3<f32>,
    t_offset: f32,
    t_timings: array<vec4<f32>, MAX_LIGHT_ALTER_COUNT>
}


@group(0) @binding(1) var<uniform> globals: Globals;

@group(1) @binding(0) var tree_sprite: texture_2d<f32>;
@group(1) @binding(1) var tree_sampler: sampler;
@group(1) @binding(2) var lights_sprite: texture_2d<f32>;
@group(1) @binding(3) var lights_sampler: sampler;
@group(1) @binding(4) var light_mask_sprite: texture_2d<f32>;
@group(1) @binding(5) var light_mask_sampler: sampler;
@group(1) @binding(6) var<uniform> light_mask_items: array<LightMaskItem, LIGHT_MASK_COUNT>;
@group(1) @binding(7) var<uniform> duration: f32;


fn overlay(base: vec4<f32>, color: vec4<f32>) -> vec4<f32> {
    return vec4(mix(base.rgb, color.rgb, color.a), 1f);
}

fn remap_if_in_range(range_from: vec2<f32>, range_to: vec2<f32>, t: f32) -> f32 {
    let remap = range_to.x + (t - range_from.x) * (range_to.y - range_to.x) / (range_from.y - range_from.x + EPSILON);
    let is_in_range = step(0f, t - range_from.x) * (1f - step(0f, t - range_from.y));
    return remap * is_in_range;
}

@fragment fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let main_color = textureSample(tree_sprite, tree_sampler, in.uv);
    let light_mask = textureSample(light_mask_sprite, light_mask_sampler, in.uv);

    if light_mask.a * (f32(LIGHT_MASK_A) + 1f) == 0 {
        return main_color;
    }

    let light_index =
        (
            (
                (
                    u32(light_mask.a * f32(LIGHT_MASK_A) - 0.25f)
                ) * LIGHT_MASK_B +
                u32(light_mask.b * (f32(LIGHT_MASK_B) - 1f) + 0.75f)
            ) * LIGHT_MASK_G +
            u32(light_mask.g * (f32(LIGHT_MASK_G) - 1f) + 0.75f)
        ) * LIGHT_MASK_R +
        u32(light_mask.r * (f32(LIGHT_MASK_R) - 1f) + 0.75f);
    

    let light_mask_item = light_mask_items[light_index];
    var light_mask_color = vec4(light_mask_item.color, 0f);

    let t = (globals.time / duration + light_mask_item.t_offset) % 1;
    for (var i = 0; i < MAX_LIGHT_ALTER_COUNT; i++) {
        let light_mask_timing = light_mask_item.t_timings[i];
        let light_opacity = remap_if_in_range(light_mask_timing.xy, light_mask_timing.zw, t);
        light_mask_color.a += light_opacity;
    }

    let lights_color = textureSample(lights_sprite, lights_sampler, in.uv) * light_mask_color;
    
    return overlay(main_color, lights_color);
}