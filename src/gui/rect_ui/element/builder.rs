use glam::{UVec2, Vec2};

use crate::{
    color::RGBA,
    gui::rect_ui::{BorderRadius, ExtraBufferData, GUIRects, Rect},
};

use super::{
    create_new_rect_element, Border, ColoringType, LinearGradient, MaskType, RadialGradient,
    TextureSlice,
};

macro_rules! into_extra_buffer {
    ($t: ident) => {
        impl Into<ExtraBufferData<$t>> for $t {
            fn into(self) -> ExtraBufferData<$t> {
                ExtraBufferData::NewData(self)
            }
        }

        impl Into<ExtraBufferData<$t>> for u16 {
            fn into(self) -> ExtraBufferData<$t> {
                ExtraBufferData::PrevIndex(self)
            }
        }
    };
}

into_extra_buffer!(RGBA);
into_extra_buffer!(BorderRadius);
into_extra_buffer!(TextureSlice);
into_extra_buffer!(RadialGradient);
into_extra_buffer!(LinearGradient);
into_extra_buffer!(Rect);

pub struct ElementBuilder {
    position: Vec2,
    size: Vec2,
    rotation: f32,
    rect_mask: Option<ExtraBufferData<Rect>>,
    mask_type: MaskType,
    coloring_type: ColoringType,
}

impl ElementBuilder {
    pub fn new( position: Vec2, size: Vec2) -> Self {
        let mask_type = MaskType::Rect { border: None };
        let coloring_type = ColoringType::Color(ExtraBufferData::NewData(RGBA::WHITE));
        Self {
            position,
            size,
            rotation: 0.0,
            rect_mask: None,
            mask_type,
            coloring_type,
        }
    }

    pub fn set_border(mut self, new_border: Option<Border>) -> Self {
        match self.mask_type {
            MaskType::Rect { ref mut border }
            | MaskType::RoundRect { ref mut border, .. }
            | MaskType::Circle { ref mut border } => {
                *border = new_border;
            }
            MaskType::TextureMask(..) | MaskType::SDFFont(..) => { /* No Op */ }
        }
        self
    }

    pub fn set_color(mut self, color: ExtraBufferData<RGBA>) -> Self {
        self.coloring_type = ColoringType::Color(color);
        self
    }
    pub fn set_texture_color(mut self, texture_slice: ExtraBufferData<TextureSlice>) -> Self {
        self.coloring_type = ColoringType::TextureColor(texture_slice);
        self
    }
    pub fn set_radial_gradient(mut self, gradient: ExtraBufferData<RadialGradient>) -> Self {
        self.coloring_type = ColoringType::RadialGradient(gradient);
        self
    }
    pub fn set_linear_gradient(mut self, gradient: ExtraBufferData<LinearGradient>) -> Self {
        self.coloring_type = ColoringType::LinearGradient(gradient);
        self
    }

    pub fn set_rect(mut self) -> Self {
        let border = self.mask_type.get_border();
        self.mask_type = MaskType::Rect { border };
        self
    }
    pub fn set_round_rect(mut self, round_rect: ExtraBufferData<BorderRadius>) -> Self {
        let border = self.mask_type.get_border();
        self.mask_type = MaskType::RoundRect {
            border_radius: round_rect,
            border,
        };
        self
    }
    pub fn set_circle(mut self) -> Self {
        let border = self.mask_type.get_border();
        self.mask_type = MaskType::Circle { border };
        self
    }
    pub fn set_sdffont(mut self, texture_slice: ExtraBufferData<TextureSlice>) -> Self {
        self.mask_type = MaskType::SDFFont(texture_slice);
        self
    }
    pub fn set_texture_mask(mut self, texture_slice: ExtraBufferData<TextureSlice>) -> Self {
        self.mask_type = MaskType::TextureMask(texture_slice);
        self
    }

    pub fn set_rect_mask(mut self, rect_mask: ExtraBufferData<Rect>) -> Self {
        self.rect_mask = Some(rect_mask);
        self
    }

    pub fn set_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn build(mut self, gui_rects: &mut GUIRects) {
        if let None = self.rect_mask{
            self.rect_mask = Some(Rect{
                position: gui_rects.screen_size.as_vec2() * 0.5,
                size: gui_rects.screen_size.as_vec2(),
            }.into());
        }
        create_new_rect_element(
            gui_rects,
            self.position,
            self.size,
            self.rotation,
            self.rect_mask.unwrap(),
            &self.mask_type,
            &self.coloring_type,
        );
    }
}
