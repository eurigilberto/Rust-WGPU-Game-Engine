//This shader is meant to be the used for the rect instances
//that needs to be rendered.
struct GlobalUIData{
	screen_width_height: vec2<f32>;
};
[[group(0), binding(0)]]
var<uniform> global_ui_data: GlobalUIData;

// Vertex shader
struct VertexOutput {
	[[location(0)]] vert_position: vec2<f32>;
	//Instance data
	[[location(1)]] rect_position: vec2<f32>;
	[[location(2)]] rect_size: vec2<f32>;
	[[location(3)]] color: vec4<f32>;
	[[location(4)]] rect_round: vec4<f32>;

	[[location(5)]] border_width: f32;
	[[location(6)]] border_color: vec4<f32>;
	//Required built in
    [[builtin(position)]] clip_position: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
	[[builtin(instance_index)]] in_instance_index: u32, 
	//provided in normalized screen space
	[[location(0)]] rect_position: vec2<f32>,
	[[location(1)]] rect_size: vec2<f32>,
	[[location(2)]] rect_depth: f32, //might not be used yet
	//provided in screen space
	[[location(3)]] rect_round: vec4<f32>,
	//misc
	[[location(4)]] color: vec4<f32>,

	[[location(5)]] border_width: f32,
	[[location(6)]] border_color: vec4<f32>,
) -> VertexOutput {
	var out: VertexOutput;
	
	var offset_list = array<vec2<f32>,4>(
		vec2<f32>(0.5, 0.5),
		vec2<f32>(-0.5, 0.5),
		vec2<f32>(0.5, -0.5),
		vec2<f32>(-0.5, -0.5)
	);

	let vertex_position_offset: vec2<f32> = offset_list[in_vertex_index];
	let position_offset: vec2<f32> = (vertex_position_offset * rect_size);
	let vertex_position: vec2<f32> = position_offset + rect_position;

    out.clip_position = vec4<f32>(vertex_position.x, vertex_position.y, rect_depth, 1.0);
	out.vert_position = vertex_position;

	let r_size = rect_size * vec2<f32>(global_ui_data.screen_width_height);
	out.rect_position = vertex_position_offset * r_size;
	out.rect_size = r_size * 0.5;

	out.color = color;
	out.rect_round = rect_round  * 2.0;
    out.border_width = border_width * 2.0;
	out.border_color = border_color;
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

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
	var dist = sd_rounded_box(in.rect_position, in.rect_size, in.rect_round);
	
	let alpha = 1.0 - clamp(dist, 0.0, 1.0);
	if(alpha < 0.0001){
		discard;
	}else{
		//let alpha = smoothStep(-0.05, 0.05, alpha);
		var border_dist = dist + in.border_width;
		border_dist = clamp(border_dist, 0.0, 1.0);
		let r_color = in.color + border_dist * (in.border_color - in.color);
		return vec4<f32>(r_color.xyz, r_color.w * alpha);
	}
	
	//return vec4<f32>(1.0,0.0,0.0, 1.0);
}