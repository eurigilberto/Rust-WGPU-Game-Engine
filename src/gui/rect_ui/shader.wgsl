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
	[[location(2)]] texture_position: vec2<f32>;
	
	//Instance data
	[[location(3)]] size: vec2<f32>;
	[[location(4)]] color: vec4<f32>;
	[[location(5)]] mask: vec4<f32>;
	[[location(6)]] border_radius: vec4<f32>;

	[[location(7)]] data_vector_0: vec4<u32>;
	[[location(8)]] data_vector_1: vec4<u32>;

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
	[[location(2)]] data_vector_1: vec4<u32>,
) -> VertexOutput {
	var out: VertexOutput;
	
	var offset_list = array<vec2<f32>,4>(
		vec2<f32>(1.0, 1.0),
		vec2<f32>(-1.0, 1.0),
		vec2<f32>(1.0, -1.0),
		vec2<f32>(-1.0, -1.0)
	);

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

	let screen_width_height: vec2<f32> = vec2<f32>(gui_render_pass_data.screen_size.x, gui_render_pass_data.screen_size.y);

	let rect_px_position = vec2<f32>(f32(position_size.x), f32(position_size.y));
	let rect_px_size = vec2<f32>(f32(position_size.z), f32(position_size.w));

	let rect_position = (rect_px_position / screen_width_height) * 2.0;
	let rect_size = rect_px_size / screen_width_height;

	let screen_origin_position: vec2<f32> = vec2<f32>(-1.0,-1.0); //in clip space

	let vertex_position_offset: vec2<f32> = offset_list[in_vertex_index];
	let position_offset: vec2<f32> = (vertex_position_offset * rect_size);

	let vertex_position: vec2<f32> = screen_origin_position + rect_position + position_offset;

    out.clip_position = vec4<f32>(vertex_position.x, vertex_position.y, 0.0, 1.0);
	out.vert_position = vertex_position;

	if(data_vector_0.y > u32(0)){
		let tx_pos_index = data_vector_0.y - u32(1);
		let tx_pos_data = texture_position.data[tx_pos_index];

		let tx_pos_start = vec2<f32>(f32(tx_pos_data.x), f32(tx_pos_data.y));
		let size_pack:u32 = tx_pos_data.z;
		let tx_width = size_pack >> u32(16);
		let tx_height = size_pack & u32(0x0000ffff);
		let tx_size = vec2<f32>(f32(tx_width), f32(tx_height));

		let tx_size_offset = tx_size_offset_mult[in_vertex_index] * tx_size;

		out.texture_position = (tx_pos_start + tx_size_offset) / vec2<f32>(1024.0, 1024.0);
	}else{
		out.texture_position = vec2<f32>(0.0, 0.0);
	}
	
	out.size = rect_px_size * 0.5;
	out.vert_px_position = vertex_position_offset * out.size;

	if(data_vector_0.w > u32(0)){
		let color_index = data_vector_0.w - u32(1);
		out.color = color.data[color_index];
	}else{
		out.color = vec4<f32>(1.0,1.0,1.0,1.0);
	}

	// Mask needs to be in the same space as the vert_position variable
	if(data_vector_0.x > u32(0)){
		let mask_index = data_vector_0.x - u32(1);
		out.mask = rect_mask.data[mask_index];
	}else{
		out.mask = vec4<f32>(-1.0,-1.0,1.0,1.0);
	}

	if(data_vector_0.z > u32(0)){
		let border_radius_index = data_vector_0.z - u32(1);
		out.border_radius = border_radius.data[border_radius_index];
	}else{
		out.border_radius = vec4<f32>(0.0,0.0,0.0,0.0);
	}

	out.data_vector_0 = data_vector_0;
	out.data_vector_1 = data_vector_1;

	return out;
}

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

	let element_type: u32 = in.data_vector_1.w & u32(0x000000ff);
	let texture_mask: u32 = in.data_vector_1.w >> u32(8);

	let tex_pos_grad = fwidth(in.texture_position);

	var mask = 1.0;
	var border_mask = 0.0;
	if(element_type == u32(0)){
		let box_dst = -(sd_rounded_box(in.vert_px_position, in.size, in.border_radius) - 0.5);
		border_mask = clamp(box_dst - f32(in.data_vector_1.z), 0.0, 1.0);
		mask = clamp(box_dst, 0.0, 1.0);
	}
	else if(element_type == u32(1)){
		let grad = length(tex_pos_grad * in.size);
		let tx_pos_index = in.data_vector_0.w - u32(1);
		let tx_pos_data = texture_position.data[tx_pos_index];
		let texel_size = 1.0 / 1024.0;
		let sampled_pixel = textureSampleLevel(texture_atlas, texture_atlas_sampler, in.texture_position, i32(tx_pos_data.w), 0.0, vec2<i32>(0, 0));
		let pixle_dist = (sampled_pixel.x * 4.0) / grad;
		//mask = clamp(0.5 - pixle_dist, 0.0, 1.0);
		let ss = smoothStep(0.0, 0.1, sampled_pixel.x);
		mask = 1.0 - ss;
	}

	// let inside_mask = in.mask.x <= in.vert_position.x && in.mask.y <= in.vert_position.y && in.mask.z >= in.vert_position.x && in.mask.w >= in.vert_position.y;
	// if(!inside_mask){
	// 	discard;
	// }

	let main_color = mix(vec4<f32>(1.0,0.0,0.0,1.0), in.color, border_mask);
	
	out.main_color = vec4<f32>(main_color.x, main_color.y, main_color.z, main_color.w * mask);
	
	var ui_mask = u32(0);
	if(step(0.1, mask) > 0.5){
		ui_mask = texture_mask;
	}
	out.ui_mask = ui_mask;

	return out;
}