//Shader required to copy a texture to the screen using the alpha value for blending
[[group(0), binding(0)]]
var copy_texture: texture_2d<f32>;
[[group(0), binding(1)]]
var copy_texture_sampler: sampler;

// Vertex shader
struct VertexOutput {
	[[location(0)]] uv: vec2<f32>;
	//Required built in
    [[builtin(position)]] clip_position: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
) -> VertexOutput {
	var out: VertexOutput;
	
	var offset_list = array<vec2<f32>,4>(
		vec2<f32>(1.0, 1.0),
		vec2<f32>(-1.0, 1.0),
		vec2<f32>(1.0, -1.0),
		vec2<f32>(-1.0, -1.0)
	);

	let vert_pos: vec2<f32> = offset_list[in_vertex_index];
	out.clip_position = vec4<f32>(vert_pos.x, vert_pos.y, 0.0, 1.0);
	out.uv = vec2<f32>((vert_pos.x + 1.0) / 2.0, (-1.0 * vert_pos.y + 1.0) / 2.0);
    
	return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
	let coord = in.uv.xy;
	let s_color = textureSample(copy_texture , copy_texture_sampler , coord);
	return s_color;
}