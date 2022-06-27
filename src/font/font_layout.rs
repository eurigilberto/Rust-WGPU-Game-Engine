use glam::{ivec2, IVec2, Vec2};

use crate::{color::RGBA, gui::rect_ui::element::TextureSlice};

use super::font_load_gpu::FontCollection;

pub struct FontElement {
    pub position: Vec2,
    pub size: Vec2,
    pub tx_slice: TextureSlice,
    pub color: RGBA,
}

//Font layout system
pub fn create_font_layout(text: &str, font_col: &FontCollection, collection_index: usize) -> Vec<FontElement> {
    let mut current_cursor = IVec2::ZERO;

    let character_map = &font_col.characters_hasmap[collection_index];
    let character_slices = &font_col.char_texture_slices[collection_index];
    let character_data = &font_col.fonts_characters[collection_index];

    let mut text_elements = Vec::with_capacity(text.len());

    for ref char in text.chars() {
        let char_id = character_map
            .get(char)
            .expect("Character now found in font definition");

        let char_data = &character_data.character_info_collection[char_id.metric_index];
        let char_position = current_cursor + ivec2(char_data.metrics.xmin, char_data.metrics.ymin);
		let char_texture_slice = character_slices[char_id.slice_index].texture_slice;

		let char_size = char_data.get_padded_size().as_vec2();

        //println!("Char to render {} - size: {}", *char, char_data.get_padded_size());

		if *char != ' ' {
			text_elements.push(FontElement {
				position: char_position.as_vec2() + (char_size * 0.5),
				size: char_size,
				tx_slice: char_texture_slice,
				color: RGBA::WHITE,
			});
		}
		current_cursor += ivec2(char_data.metrics.advance_width as i32, 0);
    }

	text_elements
}
