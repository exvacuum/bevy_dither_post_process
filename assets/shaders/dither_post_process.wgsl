#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;
@group(0) @binding(2) var threshold_map_texture: texture_2d<f32>;
@group(0) @binding(3) var threshold_map_sampler: sampler;

@fragment
fn fragment(
	in: FullscreenVertexOutput
) -> @location(0) vec4<f32> {
    let screen_size = vec2i(textureDimensions(screen_texture));
    let threshold_map_size = vec2i(textureDimensions(threshold_map_texture));
    let pixel_position = vec2i(floor(in.uv * vec2f(screen_size)));
    let map_position = vec2f(pixel_position % threshold_map_size) / vec2f(threshold_map_size);

    let threshold = textureSample(threshold_map_texture, threshold_map_sampler, map_position).r;

    let base_color = textureSample(screen_texture, screen_sampler, in.uv);
    let luma = (0.2126 * base_color.r + 0.7152 * base_color.g + 0.0722 * base_color.b);
    let value = f32(luma > threshold);

    return vec4f(value, value, value, 1.0);
}
