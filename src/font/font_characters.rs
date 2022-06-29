use std::collections::HashMap;

use glam::{UVec2, uvec2};
use half::f16;
use sdf_glyph_renderer::BitmapGlyph;

use super::font_atlas::{
    generate_sdf_bitmaps, parse_font_from_bytes, FontCharLimit, FontCreationError,
};

pub struct CharacterInfo {
    pub character: char,
    pub bitmap_padding: usize,
    pub metrics: fontdue::Metrics,
}

impl CharacterInfo {
    pub fn get_padded_size(&self) -> UVec2 {
        uvec2(self.get_padded_width(), self.get_padded_height())
    }
    pub fn get_padded_width(&self) -> u32 {
        (self.metrics.width + self.bitmap_padding * 2) as u32
    }
    pub fn get_padded_height(&self) -> u32 {
        (self.metrics.height + self.bitmap_padding * 2) as u32
    }
}

pub struct FontCharacters {
    pub sdf_bitmap_collection: Vec<Vec<f16>>,
    pub character_info_collection: Vec<CharacterInfo>,
    pub size_factor: f32,
    pub padding: usize,
}

impl FontCharacters {
    pub fn new_from_file(
        file_data: &[u8],
        character_size: f32,
        bitmap_padding: usize,
        font_char_limit: FontCharLimit,
    ) -> Result<Self, FontCreationError> {
        match parse_font_from_bytes(file_data, character_size) {
            Ok(font) => {
                
                let character_collection =
                    create_character_slices(&font, character_size, bitmap_padding, font_char_limit);

                let character_bitmaps =
                    create_character_bitmaps(&font, &character_collection, character_size, bitmap_padding);

                let bitmaps_sdf_half = generate_sdf_bitmaps(&character_bitmaps, bitmap_padding);

                Ok(Self {
                    sdf_bitmap_collection: bitmaps_sdf_half,
                    character_info_collection: character_collection,
                    size_factor: 1.0 / character_size,
                    padding: bitmap_padding
                })
            }
            Err(err) => return Err(FontCreationError::FontFileParsing(String::from(err))),
        }
    }
}

fn create_character_slices(
    font: &fontdue::Font,
    character_size: f32,
    bitmap_padding: usize,
    font_char_limit: FontCharLimit,
) -> Vec<CharacterInfo> {
    let mut char_array: Vec<char> = font.chars().keys().into_iter().map(|key| *key).collect();
    char_array.sort();
    match font_char_limit {
        FontCharLimit::All => { /* No Op */ }
        FontCharLimit::LimitedCount(count) => {
            char_array.truncate(count);
        }
    };

    let character_count = char_array.len();
    let mut slice_coords = Vec::<CharacterInfo>::with_capacity(character_count);

    for char_ref in char_array.iter() {
        let character = *char_ref;
        let char_index = font.lookup_glyph_index(character);
        let metrics = font.metrics_indexed(char_index, character_size);
        slice_coords.push(CharacterInfo {
            character,
            bitmap_padding,
            metrics,
        });
    }
    slice_coords.sort_by(|a, b| a.metrics.height.cmp(&b.metrics.height));

    slice_coords
}

fn create_character_bitmaps(
    font: &fontdue::Font,
    characters: &Vec<CharacterInfo>,
    character_size: f32,
    bitmap_padding: usize,
) -> Vec<BitmapGlyph> {
    let mut bitmaps: Vec<BitmapGlyph> = Vec::<BitmapGlyph>::with_capacity(characters.len());
    for character in characters.iter() {
        //Get character bitmap
        let char_index = font.lookup_glyph_index(character.character);

        //Add glyph data to texture
        font.metrics_indexed(char_index, character_size);
        let (_, bitmap) = font.rasterize_indexed(char_index, character_size);

        //Create intermidiate character bitmap with buffer zone
        let pad_size = character.get_padded_size();
        let mut padded_bitmap = vec![0; (pad_size.x * pad_size.y) as usize];
        let char_width = character.metrics.width;
        let char_height = character.metrics.height;

        for coord_y in 0..char_height {
            for coord_x in 0..char_width {
                let bitmap_index = coord_x + coord_y * char_width;
                let padded_bitmap_index = (coord_x + bitmap_padding) + (coord_y + bitmap_padding) * (pad_size.x as usize);
                padded_bitmap[padded_bitmap_index] = bitmap[bitmap_index];
            }
        }

        bitmaps.push(BitmapGlyph::new(
            padded_bitmap,
            char_width,
            char_height,
            bitmap_padding,
        ));
    }
    bitmaps
}
