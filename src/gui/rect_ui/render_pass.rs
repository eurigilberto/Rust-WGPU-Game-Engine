use glam::{vec4, UVec2};

use crate::render_system::{self, RenderSystem};

use super::GUIRects;

fn create_render_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    color_texture_view: &'a wgpu::TextureView,
    mask_texture_view: &'a wgpu::TextureView,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("GUI Render Pass"),
        color_attachments: &[
            Some(wgpu::RenderPassColorAttachment {
                resolve_target: None,
                view: color_texture_view,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        ..Default::default()
                    }),
                    store: true,
                },
            }),
            Some(wgpu::RenderPassColorAttachment {
                resolve_target: None,
                view: mask_texture_view,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        ..Default::default()
                    }),
                    store: true,
                },
            }),
        ],
        depth_stencil_attachment: None,
    })
}

fn draw_render_pass<'a>(
    mut render_pass: wgpu::RenderPass<'a>,
    rect_system: &'a GUIRects,
    system_bind_group: &'a wgpu::BindGroup,
) {
    render_pass.set_pipeline(&rect_system.rect_material.render_pipeline);

    render_pass.set_bind_group(0, system_bind_group, &[]);
    render_pass.set_bind_group(1, &rect_system.render_pass_data.bind_group, &[]);
    render_pass.set_bind_group(2, &rect_system.rect_collection.uniform_bind_group, &[]);
    render_pass.set_bind_group(3, &rect_system.texture_atlas.bind_group, &[]);

    render_pass.set_vertex_buffer(
        0,
        rect_system
            .rect_collection
            .rect_graphic
            .gpu_buffer
            .slice(..),
    );

    let instance_count = rect_system.rect_collection.rect_graphic.cpu_vector.len() as u32;
    render_pass.draw(0..4, 0..instance_count);
}

pub fn render_gui(
    encoder: &mut wgpu::CommandEncoder,
    rect_system: &GUIRects,
    system_bind_group: &wgpu::BindGroup,
    color_texture_view: &wgpu::TextureView,
    mask_texture_view: &wgpu::TextureView,
) {
    let render_pass = create_render_pass(encoder, color_texture_view, mask_texture_view);
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

        let width = render_system.render_window.size.x;
        let height = render_system.render_window.size.y;
        let buffer = render_system.create_buffer(
            "GUI render pass buffer",
            bytemuck::bytes_of(&[vec4(width as f32, height as f32, 0.0, 0.0)]),
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
    pub fn resize(&mut self, new_size: UVec2, render_system: &RenderSystem) {
        render_system.write_buffer(
            &self.buffer,
            0,
            bytemuck::bytes_of(&[vec4(new_size.x as f32, new_size.y as f32, 0.0, 0.0)]),
        );
    }
}
