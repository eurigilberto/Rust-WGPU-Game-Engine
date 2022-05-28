use crate::render_system::{RenderSystem, self};

use super::{system::GUIRectSystem, render_textures::GUIRenderTexture};

fn create_render_pass<'a>(
	encoder: &'a mut wgpu::CommandEncoder,
	gui_render_texture: &'a GUIRenderTexture,
) -> wgpu::RenderPass<'a> {
	encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
		label: Some("GUI Render Pass"),
		color_attachments: &[
			wgpu::RenderPassColorAttachment {
				resolve_target: None,
				view: &gui_render_texture.color_texture.texture_view,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color {
						..Default::default()
					}),
					store: true,
				},
			},
			wgpu::RenderPassColorAttachment {
				resolve_target: None,
				view: &gui_render_texture.mask_texture.texture_view,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color {
						..Default::default()
					}),
					store: true,
				},
			},
		],
		depth_stencil_attachment: None,
	})
}

fn draw_render_pass<'a>(
	mut render_pass: wgpu::RenderPass<'a>,
	rect_system: &'a GUIRectSystem,
	system_bind_group: &'a wgpu::BindGroup,
) {
	render_pass.set_pipeline(&rect_system.rect_material.render_pipeline);

	render_pass.set_bind_group(0, system_bind_group, &[]);
	render_pass.set_bind_group(1, &rect_system.render_pass_data.bind_group, &[]);
	render_pass.set_bind_group(2, &rect_system.rect_data_collection.uniform_bind_group, &[]);
	render_pass.set_bind_group(3, &rect_system.texture_atlas.bind_group, &[]);

	render_pass.set_vertex_buffer(
		0,
		rect_system
			.rect_data_collection
			.rect_graphic
			.gpu_buffer
			.slice(..),
	);

	let instance_count = rect_system
		.rect_data_collection
		.rect_graphic
		.cpu_vector
		.len() as u32;
	render_pass.draw(0..4, 0..instance_count);
}

fn render_gui(
	encoder: &mut wgpu::CommandEncoder,
	rect_system: &GUIRectSystem,
	system_bind_group: &wgpu::BindGroup,
) {
	let render_pass = create_render_pass(encoder, &rect_system.render_texture);
	draw_render_pass(render_pass, rect_system, system_bind_group);
}

pub struct GUIRenderPassData {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
}

impl GUIRenderPassData {
    pub fn new(render_system: &RenderSystem) -> Self {
        let bind_group_layout = render_system.render_window.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("GUI Render pass BGL"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            },
        );

        let width = render_system.render_window.size.width;
        let height = render_system.render_window.size.height;
        let buffer = render_system.create_buffer(
            "GUI render pass buffer",
            bytemuck::bytes_of(&[glam::vec4(width as f32, height as f32, 0.0, 0.0)]),
            render_system::uniform_usage(),
        );
        let bind_group =
            render_system
                .render_window
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("GUI Render pass BG"),
                    layout: &bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    }],
                });

        Self {
            bind_group_layout,
            bind_group,
            buffer,
        }
    }
}