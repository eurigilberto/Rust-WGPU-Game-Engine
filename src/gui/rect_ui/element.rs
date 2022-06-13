use glam::{UVec2, Vec2};

use crate::color::RGBA;

use super::{
    graphic::RectGraphic,
    system::{BorderRadius, ExtraBufferData, GUIRects, RectMask},
};

pub struct Border {
    size: u32, //?
    color: ExtraBufferData<RGBA>,
}

#[derive(Copy, Clone)]
pub struct TextureSlice {
    pub sample_component: u8,
    pub slice_position: UVec2,
    pub size: UVec2,
    pub array_index: u8,
}

impl Into<[u32; 4]> for TextureSlice {
    fn into(self) -> [u32; 4] {
        assert!(
            self.sample_component < 4,
            "Component to sample is out of range"
        );
        [
            self.slice_position.x,
            self.slice_position.y,
            (self.size.x & 0x0000ffff) << 16 | (self.size.y & 0x0000ffff),
            (self.array_index as u32) << 4 | (self.sample_component as u32),
        ]
    }
}

pub struct RadialGradient {
    pub colors: [RGBA; 2],
    pub center_position: Vec2,
    pub end_radius: f32,
    pub start_radius: f32
}

pub struct LinearGradient {
    pub colors: [RGBA; 2],
    pub start_position: Vec2,
    pub end_position: Vec2,
}

pub enum MaskType {
    Rect {
        border: Option<Border>,
    },
    RoundRect {
        border_radius: ExtraBufferData<BorderRadius>,

        border: Option<Border>,
    },

    /// This type takes the size specified to the rect
    /// It is going to take the shape of an elipse if the size x or size y are not the same
    Circle {
        border: Option<Border>,
    },

    TextureMask(ExtraBufferData<TextureSlice>),

    /// This type is the same as 'MaskType::TextureMask' but it interprets the sampled value as a distance,
    /// and also uses the 'fwidth' function with the texture position per vertex sent from the vertex shader
    /// it does not represent an increased cost as the function is called regardless of type
    SDFFont(ExtraBufferData<TextureSlice>),
}

pub enum ColoringType {
    Color(ExtraBufferData<RGBA>),

    /// Takes the RGBA color from the texture
    TextureColor(ExtraBufferData<TextureSlice>),

    //The following are gradients of 2 colors
    /// Layout of memory
    /// **Offset 0** is the first color
    /// **Offset 1** is the second color
    /// **Offset 2** is *(x,y)* center position | *(z)* distance to center
    /// Interpolates linearly and clamped
    RadialGradient(ExtraBufferData<RadialGradient>),

    /// Layout of memory
    /// **Offset 0** is the first color
    /// **Offset 1** is the second color
    /// **Offset 2** is *(x,y)* start position | *(z,w)* end position
    /// Interpolates linearly and clamped
    LinearGradient(ExtraBufferData<LinearGradient>),
    //Multiple value colors could be implemented
}

#[derive(Default)]
pub struct Element {
    pub position: UVec2,
    pub size: UVec2,

    // data vector 0 - 0 - X
    pub mask_type: u8,
    pub coloring_type: u8,
    pub rect_mask_index: u16,

    // data vector 0 - 1 - Y
    pub mask_data_index: u16,
    pub coloring_data_index: u16,

    // data vector 0 - 2 - Z
    pub border_color_index: u16,
    pub border_size: u16,

    // data vector 0 - 3 - W - EMPTY

    // data vector 1 - 0 - X
    pub rotation: f32,
}

pub fn add_border_data(border: &Option<Border>, element: &mut Element, gui_rects: &mut GUIRects) {
    match border {
        Some(border) => {
            element.border_size = border.size as u16;
            match border.color {
                ExtraBufferData::NewData(color) => {
                    let color_index = gui_rects.rect_collection.color.push_cpu(color.into());
                    element.border_color_index = color_index as u16;
                }
                ExtraBufferData::PrevIndex(color_index) => {
                    element.border_color_index = color_index as u16;
                }
            }
        }
        None => {
            element.border_size = 0;
            element.border_color_index = 0;
        }
    }
}

pub fn add_texture_slice(
    texture_slice: &ExtraBufferData<TextureSlice>,
    element: &mut Element,
    gui_rects: &mut GUIRects,
) {
    match texture_slice {
        ExtraBufferData::NewData(texture_slice) => {
            let slice_index = gui_rects
                .rect_collection
                .texture_position
                .push_cpu((*texture_slice).into());
            element.mask_data_index = slice_index as u16;
        }
        ExtraBufferData::PrevIndex(slice_index) => {
            element.mask_data_index = *slice_index;
        }
    }
}

pub fn mod_gl(a: f32, b: f32) -> f32 {
    a - b * f32::floor(a / b)
}

pub fn add_mask_type_data(mask_type: &MaskType, element: &mut Element, gui_rects: &mut GUIRects) {
    match mask_type {
        MaskType::Rect { border } => {
            //No masking it is just going to paint the quad
            element.mask_type = 0;
            add_border_data(border, element, gui_rects);
        }
        MaskType::RoundRect {
            border_radius,
            border,
        } => {
            element.mask_type = 1;
            match border_radius {
                ExtraBufferData::NewData(border_radius) => {
                    let border_radius_index = gui_rects
                        .rect_collection
                        .border_radius
                        .push_cpu((*border_radius).into());
                    element.mask_data_index = border_radius_index as u16;
                }
                ExtraBufferData::PrevIndex(index) => {
                    element.mask_data_index = *index;
                }
            }

            add_border_data(border, element, gui_rects);
        }
        MaskType::Circle { border } => {
            element.mask_type = 2;
            add_border_data(border, element, gui_rects);
        }
        MaskType::TextureMask(texture_slice) => {
            element.mask_type = 3;
            add_texture_slice(texture_slice, element, gui_rects);
        }
        MaskType::SDFFont(texture_slice) => {
            element.mask_type = 4;
            add_texture_slice(texture_slice, element, gui_rects);
        }
    }
}

fn add_coloring_type_data(
    coloring_type: &ColoringType,
    element: &mut Element,
    gui_rects: &mut GUIRects,
) {
    match coloring_type {
        ColoringType::Color(color) => {
            element.coloring_type = 0;
            match color {
                ExtraBufferData::NewData(color) => {
                    let color_index = gui_rects.rect_collection.color.push_cpu((*color).into());
                    element.coloring_data_index = color_index as u16;
                }
                ExtraBufferData::PrevIndex(color_index) => {
                    element.coloring_data_index = *color_index as u16;
                }
            }
        }
        ColoringType::TextureColor(texture_slice) => {
            element.coloring_type = 1;
            match texture_slice {
                ExtraBufferData::NewData(texture_slice) => {
                    let texture_slice_index = gui_rects
                        .rect_collection
                        .texture_position
                        .push_cpu((*texture_slice).into());
                    element.coloring_data_index = texture_slice_index as u16;
                }
                ExtraBufferData::PrevIndex(texture_slice_index) => {
                    element.coloring_data_index = *texture_slice_index as u16;
                }
            }
        }
        ColoringType::RadialGradient(radial_gradient) => {
            element.coloring_type = 2;
            match radial_gradient {
                ExtraBufferData::NewData(radial_gradient) => {
                    let radial_gradient_index = gui_rects
                        .rect_collection
                        .color
                        .push_cpu(radial_gradient.colors[0].into());
                    gui_rects
                        .rect_collection
                        .color
                        .push_cpu(radial_gradient.colors[1].into());
                    gui_rects.rect_collection.color.push_cpu([
                        radial_gradient.center_position.x,
                        radial_gradient.center_position.y,
                        radial_gradient.end_radius,
                        radial_gradient.start_radius,
                    ]);
                    element.coloring_data_index = radial_gradient_index as u16;
                }
                ExtraBufferData::PrevIndex(radial_gradient_index) => {
                    element.coloring_data_index = *radial_gradient_index as u16
                }
            }
        }
        ColoringType::LinearGradient(linear_gradient) => {
            element.coloring_type = 3;
            match linear_gradient {
                ExtraBufferData::NewData(linear_gradient) => {
                    let linear_gradient_index = gui_rects
                        .rect_collection
                        .color
                        .push_cpu(linear_gradient.colors[0].into());
                    gui_rects
                        .rect_collection
                        .color
                        .push_cpu(linear_gradient.colors[1].into());
                    gui_rects.rect_collection.color.push_cpu([
                        linear_gradient.start_position.x,
                        linear_gradient.start_position.y,
                        linear_gradient.end_position.x,
                        linear_gradient.end_position.y,
                    ]);
                    element.coloring_data_index = linear_gradient_index as u16;
                }
                ExtraBufferData::PrevIndex(linear_gradient_index) => {
                    element.coloring_data_index = *linear_gradient_index as u16;
                }
            }
        }
    }
}

pub fn create_new_rect_element(
    gui_rects: &mut GUIRects,
    screen_size: UVec2,
    position: UVec2,
    size: UVec2,
    rotation: f32,
    rect_mask: ExtraBufferData<RectMask>,
    mask_type: &MaskType,
    coloring_type: &ColoringType,
) -> Element {
    let mut element = Element::default();

    element.position = position;
    element.size = size;

    match rect_mask {
        ExtraBufferData::NewData(rect_mask) => {
            let rect_mask_index = gui_rects
                .rect_collection
                .rect_mask
                .push_cpu(rect_mask.transform_to_gpu(screen_size));
            element.rect_mask_index = rect_mask_index as u16;
        }
        ExtraBufferData::PrevIndex(rect_mask_index) => {
            element.rect_mask_index = rect_mask_index;
        }
    }

    add_mask_type_data(mask_type, &mut element, gui_rects);
    add_coloring_type_data(coloring_type, &mut element, gui_rects);

    gui_rects
        .rect_collection
        .rect_graphic
        .push_cpu(RectGraphic {
            position_size: [
                element.position.x,
                element.position.y,
                element.size.x,
                element.size.y,
            ],
            data_vector_0: [
                (element.mask_type as u32) << 24
                    | (element.coloring_type as u32) << 16
                    | (element.rect_mask_index as u32),
                (element.mask_data_index as u32) << 16 | (element.coloring_data_index as u32),
                (element.border_color_index as u32) << 16 | (element.border_size as u32),
                0,
            ],
            data_vector_1: [rotation, 0.0, 0.0, 0.0],
        });
    element
}
