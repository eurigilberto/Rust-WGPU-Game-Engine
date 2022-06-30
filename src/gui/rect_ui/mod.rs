pub mod collection;
pub mod cpu_gpu_buffer;
pub mod element;
pub mod event;
pub mod graphic;
pub mod material;
pub mod render_pass;
pub mod render_textures;
pub mod texture_atlas;

use glam::{vec2, UVec2, Vec2};

use crate::{
    math_utils::lerp_vec2,
    render_system::{render_texture::RenderTexture, RenderSystem},
    slotmap::slotmap::Slotmap,
};

use self::{
    collection::RectCollection, graphic::RectGraphic, material::RectMaterial,
    render_pass::GUIRenderPassData, render_textures::GUIRenderTexture, texture_atlas::TextureAtlas,
};

pub struct GUIRects {
    pub rect_material: RectMaterial,
    pub render_pass_data: GUIRenderPassData,
    /// Althoguh it is a public value, it is not meant to be used directly - use element builder instead
    pub rect_collection: RectCollection,
    pub texture_atlas: TextureAtlas,
    pub render_texture: GUIRenderTexture,
    pub screen_size: UVec2,
}

#[derive(Copy, Clone)]
pub enum ExtraBufferData<T> {
    NewData(T),
    PrevIndex(u16),
}

#[derive(Clone, Copy)]
pub enum BorderRadius {
    ForAll(f32),
    ForTopBottom {
        top: f32,
        bottom: f32,
    },
    ForLeftRight {
        left: f32,
        right: f32,
    },
    ForCorners {
        top_right: f32,
        bottom_right: f32,
        top_left: f32,
        bottom_left: f32,
    },
}

impl Into<[f32; 4]> for BorderRadius {
    fn into(self) -> [f32; 4] {
        match self {
            BorderRadius::ForAll(r) => [r, r, r, r],
            BorderRadius::ForTopBottom { top, bottom } => [top, bottom, top, bottom],
            BorderRadius::ForLeftRight { left, right } => [right, right, left, left],
            BorderRadius::ForCorners {
                top_right,
                bottom_right,
                top_left,
                bottom_left,
            } => [top_right, bottom_right, top_left, bottom_left],
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
}

pub struct RectBounds {
    pub max: Vec2,
    pub min: Vec2,
}

impl Into<Rect> for RectBounds {
    fn into(self) -> Rect {
        Rect {
            position: self.min + 0.5 * (self.max - self.min),
            size: self.max - self.min,
        }
    }
}

impl Into<RectBounds> for Rect {
    fn into(self) -> RectBounds {
        RectBounds {
            min: self.position - self.size * 0.5,
            max: self.position + self.size * 0.5,
        }
    }
}

impl Rect {
    pub fn offset_position(mut self, offset: Vec2) -> Self {
        self.position += offset;
        self
    }

    pub fn offset_size(mut self, offset: Vec2) -> Self {
        self.size += offset;
        self
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }
    pub fn height(&self) -> f32 {
        self.size.y
    }

    pub fn transform_to_gpu(&self, screen_size: UVec2) -> [f32; 4] {
        let start_position = vec2(-1.0, -1.0);
        let bottom_left =
            start_position + (self.position * 2.0 - self.size) / screen_size.as_vec2();
        let top_right = start_position + (self.position * 2.0 + self.size) / screen_size.as_vec2();

        [bottom_left.x, bottom_left.y, top_right.x, top_right.y]
    }

    pub fn inside_rect(&self, mouse_position: Vec2) -> bool {
        let to_mouse_pos = Vec2::abs(mouse_position - self.position);
        let half_size = self.size * 0.5;
        to_mouse_pos.x <= half_size.x && to_mouse_pos.y <= half_size.y
    }

    pub fn top_left_position(&self) -> Vec2 {
        self.position + vec2(-self.size.x * 0.5, self.size.y * 0.5)
    }

    pub fn left_position(&self) -> Vec2 {
        self.position + vec2(-self.size.x * 0.5, 0.0)
    }

    pub fn bottom_left_position(&self) -> Vec2 {
        self.position - self.size * 0.5
    }

    pub fn intersecting_rect(&self, other: &Self) -> bool {
        let rect_a: RectBounds = Rect::into(*self);
        let rect_b: RectBounds = Rect::into(*other);

        let less_any_comp_vec2 = |a: Vec2, b: Vec2| a.x < b.x || a.y < b.y;

        if less_any_comp_vec2(rect_a.max, rect_b.min) || less_any_comp_vec2(rect_b.max, rect_a.min)
        {
            return false;
        } else {
            return true;
        }
    }

    pub fn combine_rects(&self, other: &Self) -> Option<Self> {
        // transform to min mas bounds
        let rect_a: RectBounds = Rect::into(*self);
        let rect_b: RectBounds = Rect::into(*other);

        let less_any_comp_vec2 = |a: Vec2, b: Vec2| a.x < b.x || a.y < b.y;

        if less_any_comp_vec2(rect_a.max, rect_b.min) || less_any_comp_vec2(rect_b.max, rect_a.min)
        {
            None
        } else {
            let new_min = rect_a.min.max(rect_b.min);
            let new_max = rect_a.max.min(rect_b.max);

            let new_position = lerp_vec2(new_min, new_max, vec2(0.5, 0.5));
            let new_size = new_max - new_min;

            Some(Self {
                position: new_position,
                size: new_size,
            })
        }
    }
}

impl GUIRects {
    pub fn new(
        render_system: &RenderSystem,
        system_bind_group_layout: &wgpu::BindGroupLayout,
        size: UVec2,
        render_texture_slotmap: &mut Slotmap<RenderTexture>,
        initial_capacity: usize,
    ) -> Self {
        let texture_atlas = TextureAtlas::new(render_system, 1024, 1024, 2);
        let rect_collection = RectCollection::new(initial_capacity, render_system);
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
            screen_size: size,
        }
    }

    pub fn get_color_rt<'a>(&self, rt_slotmap: &'a Slotmap<RenderTexture>) -> &'a RenderTexture {
        rt_slotmap
            .get_value(&self.render_texture.color_texture_key)
            .expect("GUI Color Render Texture not found")
    }

    pub fn resize(
        &mut self,
        new_size: UVec2,
        render_system: &RenderSystem,
        render_texture_slotmap: &mut Slotmap<RenderTexture>,
    ) {
        let color_rt = render_texture_slotmap
            .get_value_mut(&self.render_texture.color_texture_key)
            .unwrap();
        color_rt.resize_texture(new_size, render_system);

        let mask_rt = render_texture_slotmap
            .get_value_mut(&self.render_texture.mask_texture_key)
            .unwrap();
        mask_rt.resize_texture(new_size, render_system);

        self.screen_size = new_size;
        self.render_pass_data.resize(new_size, render_system);
    }
}
