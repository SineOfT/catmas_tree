#import bevy_ui::ui_vertex_output::UiVertexOutput
#import "functions.wgsl"::overlay


@group(1) @binding(0) var<uniform> hue: f32;
@group(1) @binding(1) var<uniform> alpha: f32;
@group(1) @binding(2) var<uniform> whiteness: f32;
@group(1) @binding(3) var<uniform> blackness: f32;


fn gamma_function(value: f32) -> f32 {
    if value <= 0.0f {
        return value;
    }
    if value <= 0.04045f {
        return value / 12.92f; // linear falloff in dark values
    } else {
        return pow((value + 0.055f) / 1.055f, 2.4f); // gamma curve in other area
    }
}


fn calculate_color_swatch(hue: f32, whiteness: f32, blackness: f32, alpha: f32) -> vec4<f32> {
    let h = (hue / 60f) % 6f;
    let w = whiteness - blackness/2f;
    let inv_b = 1f - (blackness - whiteness/2f);
    
    let r = mix(w, inv_b, saturate( abs(h - 3f) - 1f));
    let g = mix(w, inv_b, saturate(-abs(h - 2f) + 2f));
    let b = mix(w, inv_b, saturate(-abs(h - 4f) + 2f));
    return vec4(gamma_function(r), gamma_function(g), gamma_function(b), alpha);
}


@fragment fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {    
    let color_swatch = calculate_color_swatch(hue, in.uv.y, in.uv.x, alpha);    

    let dist_to_pointer = distance(in.uv, vec2(blackness, whiteness));
    let pointer_color = calculate_color_swatch(hue, whiteness, blackness, alpha);
    let pointer = (1f-step(0.1f, dist_to_pointer)) * pointer_color;

    let color = overlay(color_swatch, pointer);

    return color;
}