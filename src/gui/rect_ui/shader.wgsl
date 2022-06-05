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
var<uniform> global_ui_data: GlobalUIData;

struct GUIRenderpassData{
	screen_size: vec4<f32>;
}
[[group(1), binding(0)]]
var<uniform> gui_render_pass_data: GUIRenderpassData;

[[group(2), binding(0)]]
var<uniform> rect_mask: array<vec4<f32>>;
[[group(2), binding(1)]]
var<uniform> border_radius: array<vec4<f32>>;
[[group(2), binding(2)]]
var<uniform> texture_position: array<vec4<u32>>;
[[group(2), binding(3)]]
var<uniform> color: array<vec4<f32>>;

[[group(3), binding(0)]]
var texture_atlas: texture_2d_array<f32>;
[[group(3), binding(1)]]
var texture_atlas_sampler: sampler;

// Vertex shader
struct VertexOutput {
	[[location(0)]] vert_position: vec2<f32>;
	//Instance data
	[[location(1)]] size: vec2<u32>;
	
	//[[location(2)]] data_vector_0: vec4<u32>,
	//[[location(3)]] data_vector_1: vec4<u32>,

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
		vec2<f32>(0.5, 0.5),
		vec2<f32>(-0.5, 0.5),
		vec2<f32>(0.5, -0.5),
		vec2<f32>(-0.5, -0.5)
	);

	let r_position = vec2<f32>(position_size.x, position_size.y);
	let r_size = vec2<f32>(position_size.z, position_size.w);

	let vertex_position_offset: vec2<f32> = offset_list[in_vertex_index];
	let position_offset: vec2<f32> = (vertex_position_offset * r_size);
	let vertex_position: vec2<f32> = position_offset + r_position;

    out.clip_position = vec4<f32>(vertex_position.x, vertex_position.y, 0.0, 1.0);
	out.vert_position = vertex_position;
	
	out.size = vec2<u32>(position_size.z, position_size.w);
	
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
	[[location(0)]] main_color: vec4<f32>,
	[[location(1)]] ui_mask: u32
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> FragmentOutput {
	//var dist = sd_rounded_box(in.rect_position, in.rect_size, in.rect_round);

	var out: FragmentOutput;

	out.main_color = vec4<f32>(1.0,0.0,0.0, 1.0);
	out.ui_mask = 0;
	
	return out;
}