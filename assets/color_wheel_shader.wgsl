#import bevy_ui::ui_vertex_output::UiVertexOutput;



@group(1) @binding(0) var<uniform> hue: f32;
@group(1) @binding(1) var<uniform> alpha: f32;


fn gamma_function(value: f32) -> f32 {
    if value <= 0.0 {
        return value;
    }
    if value <= 0.04045 {
        return value / 12.92; // linear falloff in dark values
    } else {
        return pow((value + 0.055) / 1.055, 2.4); // gamma curve in other area
    }
}

@fragment fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {    
    let tri_uv = in.uv * vec2(1f, 2f) + vec2(0f, -1f);
    
    // if tri_uv.x + abs(tri_uv.y) > 1f { return vec4(0f); }

    // let blackness = saturate(0.5f - tri_uv.x/2f + tri_uv.y);
    // let whiteness = saturate(0.5f - tri_uv.x/2f - tri_uv.y);
    
    let whiteness = saturate(in.uv.y - in.uv.x/2f);
    let blackness = saturate(in.uv.x - in.uv.y/2f);

    let w = whiteness;
    let v = 1f - blackness;

    let h = (hue % 360f) / 60f;
    let j = floor(h);
    let g = h - j;

    let i = u32(j);

    let f = mix(g, 1f - g, f32(i % 2));

    let n = mix(w, v, f);

    let a = gamma_function(v);
    let b = gamma_function(n);
    let c = gamma_function(w);


    //TODO: change to not branching
    if i == 0 { return vec4(a, b, c, alpha); }
    if i == 1 { return vec4(b, a, c, alpha); }
    if i == 2 { return vec4(c, a, b, alpha); }
    if i == 3 { return vec4(c, b, a, alpha); }
    if i == 4 { return vec4(b, c, a, alpha); }
    if i == 5 { return vec4(a, c, b, alpha); }
    
    return vec4(0f);
}