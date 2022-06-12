use glam::{vec2, UVec2};
use wgpu::Color;

use crate::{render_system::{RenderSystem, texture}, RenderTextureSlotmap};

use super::{
    collection::RectCollection, graphic::RectGraphic, material::RectMaterial,
    render_pass::GUIRenderPassData, render_textures::GUIRenderTexture, texture_atlas::TextureAtlas,
};

pub struct GUIRects {
    pub rect_material: RectMaterial,
    pub render_pass_data: GUIRenderPassData,
    pub rect_collection: RectCollection,
    pub texture_atlas: TextureAtlas,
    pub render_texture: GUIRenderTexture,
}

pub enum ElementType {
    RoundRect = 0,
    SDFFont = 1,
    Circle = 2,
}

pub enum ExtraBufferData<T> {
    NewData(T),
    PrevIndex(usize),
    None,
}

pub struct BorderRadius {
    top_right: f32,
    bottom_right: f32,
    top_left: f32,
    bottom_left: f32,
}

impl BorderRadius {
    pub fn get_data(&self) -> [f32; 4] {
        [
            self.top_right,
            self.bottom_right,
            self.top_left,
            self.bottom_left,
        ]
    }
}

pub struct RectMask {
    pub position: UVec2,
    pub size: UVec2,
}

impl RectMask {
    pub fn transform_to_gpu(&self, screen_size: UVec2) -> [f32; 4] {
        let start_position = vec2(-1.0, -1.0);
        let bottom_left = start_position
            + (self.position.as_vec2() * 2.0 - self.size.as_vec2()) / screen_size.as_vec2();
        let top_right = start_position
            + (self.position.as_vec2() * 2.0 + self.size.as_vec2()) / screen_size.as_vec2();

        [bottom_left.x, bottom_left.y, top_right.x, top_right.y]
    }
}

/*
	- **vec4\<u32\>** 
		- **x , y** top left corner position of the `texture slice`.
		- **z** a packed u32 that holds the `size` of the `texture slice`.
		- **w** the texture array selection and the component selection 4 LSB.
 */

pub enum SampleComponentSelection{
    X = 0,
    Y = 1,
    Z = 2,
    W = 3,
    FullColor = 4
}

pub struct TexturePosition{
    pub position: UVec2,
    pub size: UVec2,
    pub texture_array_selection: u32,
    pub component_selection: SampleComponentSelection
}

impl GUIRects {
    pub fn new(
        render_system: &RenderSystem,
        system_bind_group_layout: &wgpu::BindGroupLayout,
        size: UVec2,
        render_texture_slotmap: &mut RenderTextureSlotmap,
    ) -> Self {
        let texture_atlas = TextureAtlas::new(render_system, 1024, 1024, 2);
        let rect_collection = RectCollection::new(126, render_system);
        let render_pass_data = GUIRenderPassData::new(render_system);

        let rect_material = RectMaterial::new(
            render_system,
            &[
                &system_bind_group_layout,
                &render_pass_data.bind_group_layout,
                &rect_collection.uniform_bind_group_layout,
                &texture_atlas.bind_group_layout,
            ],
            &[RectGraphic::get_vertex_buffer_layout()],
        );

        let render_texture =
            GUIRenderTexture::new(render_system, size.x, size.y, render_texture_slotmap);

        Self {
            rect_material,
            render_pass_data,
            rect_collection,
            texture_atlas,
            render_texture,
        }
    }

    pub fn resize(&mut self, new_size: UVec2, render_system: &RenderSystem) {
        self.render_pass_data.resize(new_size, render_system);
    }

    pub fn push_round_rect(
        &mut self,
        color: ExtraBufferData<Color>,
        border_radius: ExtraBufferData<BorderRadius>,
        rect_mask: ExtraBufferData<RectMask>,
        texture_position: ExtraBufferData<TexturePosition>
    ) -> bool {
        self.rect_collection
            .color
            .cpu_vector
            .push([0.5, 0.5, 0.5, 1.0]);
        self.rect_collection
            .rect_mask
            .cpu_vector
            .push([10.0, 20.0, 30.0, 40.0]);
        self.rect_collection
            .texture_position
            .cpu_vector
            .push([1, 2, 3, 4]);
        self.rect_collection
            .border_radius
            .cpu_vector
            .push([11.0, 11.0, 0.0, 11.0]);
        false
    }
}
