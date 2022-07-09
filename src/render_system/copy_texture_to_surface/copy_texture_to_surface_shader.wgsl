//Shader required to copy a texture to the screen using the alpha value for blending
@group(0) @binding(0)
var copy_texture: texture_2d<f32>;
@group(0) @binding(1)
var copy_texture_sampler: sampler;

// Vertex shader
struct VertexOutput {
	@location(0) uv: vec2<f32>,
	//Required built in
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
	var out: VertexOutput;
	
	var vert_list = array<vec2<f32>,4>(
		vec2<f32>(1.0, 1.0),
		vec2<f32>(-1.0, 1.0),
		vec2<f32>(1.0, -1.0),
		vec2<f32>(-1.0, -1.0)
	);

	var uv_list = array<vec2<f32>,4>(
		vec2<f32>(1.0, 0.0),
		vec2<f32>(0.0, 0.0),
		vec2<f32>(1.0, 1.0),
		vec2<f32>(0.0, 1.0)
	);

	let vert_pos: vec2<f32> = vert_list[in_vertex_index];
	out.clip_position = vec4<f32>(vert_pos.x, vert_pos.y, 0.0, 1.0);
	out.uv = uv_list[in_vertex_index];
    
	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	let coord = in.uv.xy;
	let color_sample = textureSample(copy_texture , copy_texture_sampler , coord);
	let color_only = vec3<f32>(color_sample.x, color_sample.y, color_sample.z);
	let color_corrected = pow(color_only, vec3<f32>(2.2, 2.2, 2.2));
	let final_color = vec4<f32>(color_corrected.x, color_corrected.y, color_corrected.z, color_sample.w);
	return final_color;
}