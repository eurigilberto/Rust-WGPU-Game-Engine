use crate::color;
use crate::gui::renderable_elements::rect::RectGraphic;
use crate::render_system::RenderSystem;
pub struct RectCollection {
    collection: Vec<RectGraphic>,
    buffer_capacity: usize,
    pub collection_buffer: wgpu::Buffer,
}

impl RectCollection {
    pub fn new(initial_capacity: usize, render_system: &RenderSystem) -> Self{
        let collection = vec![
            RectGraphic {
                rect_position: [0.0, 0.0],
                rect_size: [0.0, 0.0],
                rect_depth: 0.0,
                rect_round: [0.0, 0.0, 0.0, 0.0],
                color: color::RGBA::TRANSPARENT.into(),
                border_size: 0.0,
                border_color: color::RGBA::TRANSPARENT.into()
            };
            initial_capacity
        ];
        let buffer_capacity = initial_capacity;
        let collection_buffer = render_system.create_vertex_buffer(
            "RectCollection",
            bytemuck::cast_slice(collection.as_slice()),
            true,
        );

        Self{
            collection,
            buffer_capacity,
            collection_buffer
        }
    }
}
