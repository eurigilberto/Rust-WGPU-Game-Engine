use glam::{ivec2, IVec2, Vec2, vec2};

use crate::{
    color::RGBA,
    gui::rect_ui::{element::TextureSlice, Rect},
};

use super::font_load_gpu::FontCollection;

pub struct FontElement {
    pub rect: Rect,
    pub tx_slice: TextureSlice,
}

//Font layout system
pub fn create_font_layout(
    text: &str,
    font_size: f32,
    font_col: &FontCollection,
    collection_index: usize,
) -> (Vec<FontElement>, Rect) {
    let character_map = &font_col.characters_hasmap[collection_index];
    let character_slices = &font_col.char_texture_slices[collection_index];
    let character_data = &font_col.fonts_characters[collection_index];

    let mut current_cursor = IVec2::ZERO;
    let mut height: f32 = 0.0;

    let mut text_elements = Vec::with_capacity(text.len());
    let size_factor = font_size * character_data.size_factor;
    let pos_offset = -vec2(character_data.padding as f32, character_data.padding as f32) * size_factor;

    for ref char in text.chars() {
        let char_id = character_map
            .get(char)
            .expect("Character now found in font definition");

        let char_data = &character_data.character_info_collection[char_id.metric_index];
        let char_position = current_cursor
            + (ivec2(char_data.metrics.xmin, char_data.metrics.ymin).as_vec2() * size_factor)
                .as_ivec2();
        let char_texture_slice = character_slices[char_id.slice_index].texture_slice;

        let char_size = char_data.get_padded_size().as_vec2() * size_factor;
        let char_height = (char_data.metrics.height as i32 + char_data.metrics.ymin) as f32 * size_factor;
        if height < char_height {
            height = char_height;
        }

        if *char != ' ' {
            text_elements.push(FontElement {
                rect: Rect{
                    position: char_position.as_vec2() + (char_size * 0.5) + pos_offset,
                    size: char_size,
                },
                tx_slice: char_texture_slice,
            });
        }
        current_cursor += ivec2((char_data.metrics.advance_width * size_factor) as i32, 0);
    }

    let box_size = current_cursor.as_vec2() + vec2(0.0, height);
    (
        text_elements,
        Rect {
            position: box_size * 0.5,
            size: box_size,
        },
    )
}
