fn overlay(base: vec4<f32>, color: vec4<f32>) -> vec4<f32> {
    return vec4(mix(base.rgb, color.rgb, color.a), 1f);
}