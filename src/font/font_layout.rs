use std::collections::HashMap;

use fontdue::layout::CharacterData;
use glam::{ivec2, vec2, IVec2, Vec2};

use crate::{
    color::RGBA,
    font::font_characters::CharacterInfo,
    gui::rect_ui::{element::TextureSlice, Rect},
};

use super::{
    font_characters::FontCharacters,
    font_load_gpu::{CharIndices, CharTextureSlice, FontCollection},
};

#[derive(Clone, Copy)]
pub struct FontElement {
    pub rect: Rect,
    pub tx_slice: TextureSlice,
}

macro_rules! get_char_data {
    ($chr:ident, $cm:ident, $chd:ident, $cs:ident, $v1: ident, $v2: ident) => {
        let char_id = $cm
            .get($chr)
            .expect("Character now found in font definition");
        $v1 = &$chd.character_info_collection[char_id.metric_index];
        $v2 = $cs[char_id.slice_index].texture_slice;
    };
}

#[inline]
fn char_position_offset(char_data: &CharacterInfo, size_factor: f32) -> Vec2 {
    ivec2(char_data.metrics.xmin, char_data.metrics.ymin).as_vec2() * size_factor
}

//Font layout system
pub fn create_single_line(
    text: &str,
    font_size: f32,
    font_col: &FontCollection,
    collection_index: usize,
    char_spacing: f32,
) -> (Vec<FontElement>, Rect) {
    let character_map = &font_col.characters_hasmap[collection_index];
    let character_slices = &font_col.char_texture_slices[collection_index];
    let character_data = &font_col.fonts_characters[collection_index];

    let mut current_cursor = IVec2::ZERO;
    let mut height: f32 = 0.0;

    let mut text_elements = Vec::with_capacity(text.len());
    let size_factor = font_size * character_data.size_factor;
    let pos_offset = Vec2::splat(character_data.padding as f32) * size_factor;

    for ref chr in text.chars() {
        let char_data: &CharacterInfo;
        let char_texture_slice: TextureSlice;
        get_char_data!(
            chr,
            character_map,
            character_data,
            character_slices,
            char_data,
            char_texture_slice
        );

        let char_position =
            current_cursor.as_vec2() + char_position_offset(char_data, size_factor) - pos_offset;
        let char_size = char_data.get_padded_size().as_vec2() * size_factor;

        let char_height =
            (char_data.metrics.height as i32 + char_data.metrics.ymin) as f32 * size_factor;

        if height < char_height {
            height = char_height;
        }

        if *chr != ' ' {
            text_elements.push(FontElement {
                rect: Rect {
                    position: char_position + (char_size * 0.5),
                    size: char_size,
                },
                tx_slice: char_texture_slice,
            });
        }
        current_cursor += ivec2(
            (char_data.metrics.advance_width * size_factor * (1.0 + char_spacing)) as i32,
            0,
        );
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

#[derive(Default, Clone, Copy)]
pub struct WordRect {
    pub rect: Rect,
    pub index: usize,
    pub len: usize,
}

pub fn create_multi_line(
    text: &str,
    font_size: f32,
    font_col: &FontCollection,
    collection_index: usize,

    char_spacing: f32,
    line_width: f32,
    line_height: f32,
    paragraph_separation: f32,
) -> (Vec<FontElement>, Vec<WordRect>, f32) {
    let character_map = &font_col.characters_hasmap[collection_index];
    let character_slices = &font_col.char_texture_slices[collection_index];
    let character_data = &font_col.fonts_characters[collection_index];

    let mut text_elements = Vec::<FontElement>::with_capacity(text.len());

    let word_count = text.split('\n').into_iter().fold(0, |acc, paragraph| {
        let count = paragraph.split(' ').into_iter().fold(0, |acc, _| acc + 1);
        acc + count
    });

    let mut word_elements = Vec::with_capacity(word_count as usize);
    let size_factor = font_size * character_data.size_factor;
    let padding_offset = Vec2::splat(character_data.padding as f32) * size_factor;

    let space_width = {
        let space_chr = &' ';
        let char_data: &CharacterInfo;
        let char_texture_slice: TextureSlice;
        get_char_data!(
            space_chr,
            character_map,
            character_data,
            character_slices,
            char_data,
            char_texture_slice
        );
        char_data.metrics.advance_width * size_factor
    };

    let mut current_cursor = Vec2::ZERO;
    let mut word_font_elements = Vec::<FontElement>::with_capacity(20);
    for paragraphs in text.split('\n') {
        for (index, words) in paragraphs.split(' ').into_iter().enumerate() {
            let mut h_pos = 0.0;
            let mut height = 0.0;
            //Create the rect of the word then check if a new line needs to be created or not
            for ref chr in words.chars() {
                let char_data: &CharacterInfo;
                let char_texture_slice: TextureSlice;
                get_char_data!(
                    chr,
                    character_map,
                    character_data,
                    character_slices,
                    char_data,
                    char_texture_slice
                );

                let char_position = vec2(h_pos, 0.0) + char_position_offset(char_data, size_factor)
                    - padding_offset;
                let char_size = char_data.get_padded_size().as_vec2() * size_factor;

                word_font_elements.push(FontElement {
                    rect: Rect {
                        position: char_position + char_size * 0.5,
                        size: char_size,
                    },
                    tx_slice: char_texture_slice,
                });

                h_pos += char_data.metrics.advance_width * size_factor * (1.0 + char_spacing);
                let char_height =
                    (char_data.metrics.height as f32 + char_data.metrics.ymin as f32) * size_factor;
                if height < char_height {
                    height = char_height;
                }
            }
            //Word rect could be created here, only creates a new line if it is not the first word in the paragraph
            if index != 0 && line_width < current_cursor.x + h_pos {
                //The word does not fit in the current line
                current_cursor.x = 0.0;
                current_cursor.y -= line_height
            }

            //Using the current cursor, offset the font elements
            for w_elem in word_font_elements.iter_mut() {
                w_elem.rect.position += current_cursor;
            }
            word_elements.push(WordRect {
                rect: Rect {
                    position: current_cursor + vec2(h_pos, height) * 0.5,
                    size: vec2(h_pos, height),
                },
                index: text_elements.len(),
                len: word_font_elements.len(),
            });
            text_elements.extend_from_slice(&word_font_elements.as_slice());

            current_cursor.x += h_pos + space_width;
            word_font_elements.clear();
        }
        //new paragraph
        current_cursor.x = 0.0;
        current_cursor.y -= paragraph_separation;
    }
    (text_elements, word_elements, f32::abs(current_cursor.y))
}
