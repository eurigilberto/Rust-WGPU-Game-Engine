//This shader is meant to be the used for the rect instances
//that needs to be rendered.
struct SystemData{
	//time
    //delta_time
    //time_milis
    //delta_time_milis
	time_data: vec4<f32>;
};
[[group(0), binding(0)]]
var<uniform> global_ui_data: SystemData;

struct GUIRenderpassData{
	screen_size: vec4<f32>;
};
[[group(1), binding(0)]]
var<uniform> gui_render_pass_data: GUIRenderpassData;

struct VecFloatStorage{
	data: array<vec4<f32>>;
};
struct VecUIntStorage{
	data: array<vec4<u32>>;
};
[[group(2), binding(0)]]
var<storage> rect_mask: VecFloatStorage;
[[group(2), binding(1)]]
var<storage> border_radius: VecFloatStorage;
[[group(2), binding(2)]]
var<storage> texture_position: VecUIntStorage;
[[group(2), binding(3)]]
var<storage> color: VecFloatStorage;

[[group(3), binding(0)]]
var texture_atlas: texture_2d_array<f32>;
[[group(3), binding(1)]]
var texture_atlas_sampler: sampler;

// Vertex shader
struct VertexOutput {
	[[location(0)]] vert_position: vec2<f32>;
	[[location(1)]] vert_px_position: vec2<f32>;

	//it could be a texture position changing per vertex or the border radius
	[[location(2)]] masking_data: vec4<f32>;
	//it could be a color, the texture position changing per vertex,
	//the start - end position of the linear gradient,
	//or the center position and radius of a radial gradient
	[[location(3)]] coloring_data: vec4<f32>;
	
	//Instance data
	[[location(4), interpolate(flat)]] half_size: vec2<f32>;
	[[location(5), interpolate(flat)]] mask: vec4<f32>;

	// x - mask type
	// y - color type
	// z - border color index
	// w - border size
	[[location(6), interpolate(flat)]] data_vector_0: vec4<u32>;

	// if mask type requires a texture
	// x - array index | y - sample component
	// if coloring type requires a texture
	// z - array index | w - sample component
	// if coloring type is gradient
	// z - color data index
	[[location(7), interpolate(flat)]] texture_extra_data: vec4<u32>;

	//Required built in
    [[builtin(position)]] clip_position: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
	[[builtin(instance_index)]] in_instance_index: u32, 
	
	//provided in normalized screen space
	[[location(0)]] position_size: vec4<u32>,
	[[location(1)]] data_vector_0: vec4<u32>,
	[[location(2)]] data_vector_1: vec4<f32>,
) -> VertexOutput {
	var out: VertexOutput;
	
	var offset_list = array<vec2<f32>,4>(
		vec2<f32>(1.0, 1.0),
		vec2<f32>(-1.0, 1.0),
		vec2<f32>(1.0, -1.0),
		vec2<f32>(-1.0, -1.0)
	);

	// Computing Vertex position
	let screen_width_height: vec2<f32> = vec2<f32>(gui_render_pass_data.screen_size.x, gui_render_pass_data.screen_size.y);

	let rect_px_position = vec2<f32>(f32(position_size.x), f32(position_size.y));
	let rect_px_size = vec2<f32>(f32(position_size.z), f32(position_size.w));

	let rect_position = (rect_px_position / screen_width_height) * 2.0;

	let screen_origin_position: vec2<f32> = vec2<f32>(-1.0,-1.0); //in clip space

	let vertex_position_offset: vec2<f32> = offset_list[in_vertex_index];
	let position_offset: vec2<f32> = (vertex_position_offset * rect_px_size);

	//Creating rotation matrix
	let rotation = data_vector_1.x;
	let cos_rot = cos(rotation);
	let sin_rot = sin(rotation);
	let rotation_mat = mat2x2<f32>(cos_rot, -sin_rot, sin_rot, cos_rot);

	let rotated_position_offset = rotation_mat * position_offset;

	let norm_position_offset = rotated_position_offset / screen_width_height;

	let vertex_position: vec2<f32> = screen_origin_position + rect_position + norm_position_offset;

    out.clip_position = vec4<f32>(vertex_position.x, vertex_position.y, 0.0, 1.0);
	out.vert_position = vertex_position;
	///////////////////////////////////////////////////////////

	// 2 ---- 1
	// |\     |
	// | ---  |
	// |    \ |
	// 4 ---- 3 
	var tx_size_offset_mult = array<vec2<f32>,4>(
		vec2<f32>(1.0, 0.0),
		vec2<f32>(0.0, 0.0),
		vec2<f32>(1.0, 1.0),
		vec2<f32>(0.0, 1.0)
	);

	let mask_type = data_vector_0.x >> 24u;
	let coloring_type = (data_vector_0.x & 0x00ff0000u) >> 16u;
	let rect_mask_index = data_vector_0.x & 0x0000ffffu;

	let mask_data_index = data_vector_0.y >> 16u;
	let coloring_data_index = data_vector_0.y & 0x0000ffffu;

	let border_color_index = data_vector_0.z >> 16u;
	let border_size = data_vector_0.z & 0x0000ffffu;

	out.half_size = rect_px_size * 0.5;
	out.mask = rect_mask.data[rect_mask_index];
	out.texture_extra_data = vec4<u32>(0u,0u,0u,0u);

	// Get data for mask type
	if(mask_type == 1u){
		out.masking_data = border_radius.data[mask_data_index];
	}
	else if(mask_type == 3u || mask_type == 4u){
		//if mask type is texture or sdf font
		//then mask_data_index points to some data in the texture_position buffer

		let tx_data = texture_position.data[mask_data_index];
		let top_left_position = vec2<f32>(f32(tx_data.x), f32(tx_data.y)); //in pixels
		let slice_size = vec2<f32>(
			f32(tx_data.z >> 16u),
			f32(tx_data.z & 0x0000ffffu)
		);

		let tx_size_offset = tx_size_offset_mult[in_vertex_index] * slice_size;
		let tx_position = (top_left_position + tx_size_offset) / vec2<f32>(1024.0, 1024.0);

		out.masking_data = vec4<f32>(tx_position.x, tx_position.y, 0.0, 0.0);
		
		out.texture_extra_data.x = tx_data.w >> 4u;
		out.texture_extra_data.y = tx_data.w & 0x0000000fu;
	}
	else{
		out.masking_data = vec4<f32>(0.0, 0.0, 0.0, 0.0);
	}

	// Get data for coloring type
	if(coloring_type == 0u){
		out.coloring_data = color.data[coloring_data_index];
	}
	else if(coloring_type == 1u){

		let tx_data = texture_position.data[coloring_data_index];
		let top_left_position = vec2<f32>(f32(tx_data.x), f32(tx_data.y)); //in pixels
		let slice_size = vec2<f32>(
			f32(tx_data.z >> 16u),
			f32(tx_data.z & 0x0000ffffu)
		);

		let tx_size_offset = tx_size_offset_mult[in_vertex_index] * slice_size;
		let tx_position = (top_left_position + tx_size_offset) / vec2<f32>(1024.0, 1024.0);

		out.coloring_data = vec4<f32>(tx_position.x, tx_position.y, 0.0, 0.0);
		out.texture_extra_data.z = tx_data.w >> 4u;
		out.texture_extra_data.w = tx_data.w & 0x0000000fu;
	}
	else if(coloring_type == 2u || coloring_type == 3u){
		out.coloring_data = color.data[coloring_data_index + 2u];
		out.texture_extra_data.z = coloring_data_index;
	}
	else{
		out.coloring_data = vec4<f32>(0.0, 0.0, 0.0, 0.0);
	}
	
	out.half_size = rect_px_size * 0.5;
	out.vert_px_position = vertex_position_offset * out.half_size;

	out.data_vector_0 = vec4<u32>(
		mask_type,
		coloring_type,
		border_color_index,
		border_size
	);

	return out;
}

// Taken from https://www.shadertoy.com/view/4llXD7 by Inigo Quilez
fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32{
	var round = r.zw;
	
	if(p.x > 0.0){
		round = r.xy;
	}

	let r_x = round.x;
	round.x = round.y;
	if (p.y>0.0){
		round.x = r_x;
	}
    
	let q = abs(p) - b + round.x;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0,0.0))) - round.x;
}

struct FragmentOutput {
	[[location(0)]] main_color: vec4<f32>;
	[[location(1)]] ui_mask: u32;
};

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> FragmentOutput {
	var out: FragmentOutput;

	// x - mask type
	// y - color type
	// z - border color index
	// w - border size
	// data_vector_0

	let mask_type = in.data_vector_0.x;
	let coloring_type = in.data_vector_0.y;
	let border_color_index = in.data_vector_0.z;
	let border_size = in.data_vector_0.y;

	let mask_texture_position = vec2<f32>(in.masking_data.x, in.masking_data.y);
	let fwidth_mask_data = fwidth(mask_texture_position);

	var mask = 1.0;
	var border_mask = 1.0;
	
	let inside_mask = in.mask.x < in.vert_position.x && in.mask.y < in.vert_position.y && in.mask.z > in.vert_position.x && in.mask.w > in.vert_position.y;
	if(!inside_mask){
	 	discard;
	}

	// if mask type requires a texture
	// x - array index | y - sample component
	// if coloring type requires a texture
	// z - array index | w - sample component
	// texture_extra_data

	if(mask_type == 1u){//Round rect mask
		let box_dst = -(sd_rounded_box(in.vert_px_position, in.half_size, in.masking_data) - 0.5);
		mask = clamp(box_dst, 0.0, 1.0);
	}else if(mask_type == 2u){//Circle mask
		let distance_to_point = length(in.vert_px_position);
		var radius = 0.0;
		if(in.half_size.y < in.half_size.x) {
			let radius_interpolator = abs(in.vert_px_position.y / in.half_size.y);
			let corrected_lerp = radius_interpolator * radius_interpolator;
			radius = mix(in.half_size.x, in.half_size.y, corrected_lerp);
		}else{
			let radius_interpolator = abs(in.vert_px_position.x / in.half_size.x);
			let corrected_lerp = radius_interpolator * radius_interpolator;
			radius = mix(in.half_size.y, in.half_size.x, corrected_lerp);
		}
		mask = clamp(-(distance_to_point - radius - 0.5), 0.0, 1.0);
	}else if(mask_type == 3u || mask_type == 4u){//Texture mask
		let array_index = in.texture_extra_data.x;
		let sample_component = in.texture_extra_data.y;
		let sampled_pixel = textureSampleLevel(texture_atlas, texture_atlas_sampler, mask_texture_position, 
			i32(array_index), 0.0, vec2<i32>(0, 0));
		let sample = sampled_pixel[sample_component];

		if(mask_type == 3u){
			mask = clamp(sample, 0.0, 1.0);
		}
		else if(mask_type == 4u){
			let grad = length(fwidth_mask_data) * 100.0;
			let pixel_dist = (sample * 0.75) / grad;
			mask = clamp(0.5 - pixel_dist, 0.0, 1.0);
		}
	}

	let coloring_texture_position = vec2<f32>(in.coloring_data.x, in.coloring_data.y);
	var main_color = vec4<f32>(1.0,1.0,1.0,1.0);
	if(coloring_type == 0u){
		main_color = in.coloring_data;
	}
	else if(coloring_type == 1u){
		let array_index = in.texture_extra_data.z;
		let sampled_color = textureSampleLevel(texture_atlas, texture_atlas_sampler, coloring_texture_position, 
			i32(array_index), 0.0, vec2<i32>(0, 0));
		main_color = sampled_color;
	}
	else if(coloring_type == 2u){
		let color_index = in.texture_extra_data.z;
		let color_0 = color.data[color_index];
		let color_1 = color.data[color_index + 1u];

		let center = vec2<f32>(in.coloring_data.x, in.coloring_data.y);
		let vec_from_center = in.vert_px_position - center;
		let dist = length(vec_from_center);
		let interp = clamp((dist - in.coloring_data.w) / (in.coloring_data.z - in.coloring_data.w), 0.0, 1.0);

		main_color = mix(color_0, color_1, interp);
	}
	else if(coloring_type == 3u){
		let color_index = in.texture_extra_data.z;
		let color_0 = color.data[color_index];
		let color_1 = color.data[color_index + 1u];

		let start = vec2<f32>(in.coloring_data.x, in.coloring_data.y);
		let end = vec2<f32>(in.coloring_data.z, in.coloring_data.w);

		let start_end = end - start;
		let norm_start_end = normalize(start_end);
		let len_start_end = length(start_end);

		let position = in.vert_px_position - start;
		let interp = clamp(dot(position, norm_start_end) / len_start_end, 0.0, 1.0);

		main_color = mix(color_0, color_1, interp);
	}
	else{
		discard;
	}
	out.main_color = vec4<f32>(main_color.x, main_color.y, main_color.z, main_color.w * mask);

	// Background debugging, just in case something does not make sense
	//let main_color_w_bg = mix(vec4<f32>(1.0,0.0,0.0,1.0), main_color, main_color.w * mask);
	//out.main_color = main_color_w_bg;
	
	// var ui_mask = u32(0);
	// if(step(0.1, mask) > 0.5){
	// 	ui_mask = texture_mask;
	// }
	out.ui_mask = 5u;

	return out;
}