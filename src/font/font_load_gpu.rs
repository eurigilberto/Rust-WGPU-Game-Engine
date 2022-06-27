use std::{collections::HashMap, num::NonZeroU32};

use glam::{UVec2, UVec3, uvec2};
use half::{prelude::HalfFloatSliceExt, f16};
use wgpu::{Origin3d, ImageCopyTexture, TextureAspect};

use crate::gui::rect_ui::element::TextureSlice;

use super::{font_atlas::FontCharLimit, font_characters::FontCharacters};

pub struct CharIndices {
    pub metric_index: usize,
    pub slice_index: usize,
}

pub struct CharTextureSlice {
    pub char_index: usize,
    pub texture_slice: TextureSlice,
}

pub struct FontCollection {
    /* The font collection is going to contain a limited set of fonts and their position inside the gui texture atlas */
    pub fonts_characters: Vec<FontCharacters>,
    pub characters_hasmap: Vec<HashMap<char, CharIndices>>,
    pub char_texture_slices: Vec<Vec<CharTextureSlice>>,
}

pub struct FontDataLoad<'a> {
    pub name: &'a str,
    pub data: &'a [u8],
    pub char_limit: FontCharLimit,
    pub character_size: f32,
    pub padding: usize,
}

pub fn write_font_to_gpu(
    queue: &wgpu::Queue,
    gui_texture_atlas: &wgpu::Texture,
    font_data: &[FontDataLoad],
    texture_slice_size: UVec2,
    texture_slice_index: u32,
) -> Result<FontCollection, ()> {
    //Create font characters
    let mut fonts_char_collection = Vec::with_capacity(font_data.len());
    for data in font_data {
        match FontCharacters::new_from_file(
            data.data,
            data.character_size,
            data.padding,
            data.char_limit,
        ) {
            Ok(font_chars) => {
                fonts_char_collection.push(font_chars);
            }
            Err(_) => return Err(()),
        }
    }

    struct CharInfoIndex {
        collection_index: usize,
        char_index: usize,
        char_height: usize,
    }

    //Upload font characters to texture atlas
    // 1 . Order characters by height
    let character_count = fonts_char_collection
        .iter()
        .fold(0, |acc, x| acc + x.character_info_collection.len());
    let mut font_chars_height = Vec::with_capacity(character_count);
    for (coll_index, f_char) in fonts_char_collection.iter().enumerate() {
        for (char_index, char_info) in f_char.character_info_collection.iter().enumerate() {
            font_chars_height.push(CharInfoIndex {
                collection_index: coll_index,
                char_index,
                char_height: char_info.metrics.height,
            });
        }
    }
    font_chars_height.sort_by(|char_a, char_b| char_a.char_height.cmp(&char_b.char_height));

    // 2 . Add characters to texture atlas in order
    let mut font_collection_texture_slices: Vec<Vec<CharTextureSlice>> = Vec::new();
    for collection in fonts_char_collection.iter() {
        font_collection_texture_slices.push(Vec::with_capacity(
            collection.character_info_collection.len(),
        ));
    }

    let mut font_texture =
        vec![f16::from_f32(0.0); (texture_slice_size.x * texture_slice_size.y * 4) as usize];
    let mut cursor_position = UVec3::ZERO;
    let mut current_line_height = 0;
    for char_index in font_chars_height.iter() {
        //Get char width
        let char_info = &fonts_char_collection[char_index.collection_index]
            .character_info_collection[char_index.char_index];
        let char_bitmap = &fonts_char_collection[char_index.collection_index].sdf_bitmap_collection
            [char_index.char_index];

        if char_info.get_padded_width() + cursor_position.x > texture_slice_size.x {
            //Does not fit in the current line
            cursor_position.x = 0;
            cursor_position.y += current_line_height;
            current_line_height = 0;

            if cursor_position.y + char_info.get_padded_height() > texture_slice_size.y {
                //Does not fit in current texture slice
                cursor_position.y = 0;
                cursor_position.z += 1;
                println!("Increased z value");
                if cursor_position.z >= 4 {
                    panic!("Current font collection does not fit in texture slice")
                }
            }
        }

        let pad_size = char_info.get_padded_size();

        if pad_size.y > current_line_height {
            current_line_height = pad_size.y;
        }

        for coord_y in 0..pad_size.y {
            for coord_x in 0..pad_size.x {
                let px_index = coord_x + coord_y * pad_size.x;
                let sample = char_bitmap[px_index as usize];

                let texture_index = cursor_position.z
                    + (cursor_position.x + coord_x) * 4
                    + (cursor_position.y + coord_y) * 4 * texture_slice_size.x;

                font_texture[texture_index as usize] = sample;
            }
        }

        font_collection_texture_slices[char_index.collection_index].push(CharTextureSlice {
            char_index: char_index.char_index,
            texture_slice: TextureSlice {
                sample_component: cursor_position.z as u8,
                slice_position: uvec2(cursor_position.x, cursor_position.y),
                size: pad_size,
                array_index: texture_slice_index as u8,
            },
        });

        cursor_position.x += pad_size.x;
    }

    // 3 . Create a character hashmap per font, that links the slice location with the data location
    let mut font_collection_maps: Vec<HashMap<char, CharIndices>> = Vec::new();
    for _ in fonts_char_collection.iter() {
        font_collection_maps.push(HashMap::new())
    }

    for (map, (tx_slice, char_collection)) in font_collection_maps.iter_mut().zip(
        font_collection_texture_slices
            .iter()
            .zip(fonts_char_collection.iter()),
    ) {
        for (slice_index, slice) in tx_slice.iter().enumerate() {
            let character = char_collection.character_info_collection[slice.char_index].character;
            map.insert(
                character,
                CharIndices {
                    metric_index: slice.char_index,
                    slice_index: slice_index,
                },
            );
        }
    }

    // 4 . Write data to texture
    let tx_block_size = (wgpu::TextureFormat::Rgba16Float)
        .describe()
        .block_size;
    let bytes_per_row = tx_block_size as u32 * texture_slice_size.x;
    let font_texture_slice = HalfFloatSliceExt::reinterpret_cast(font_texture.as_slice());

    queue.write_texture(
        ImageCopyTexture {
            texture: gui_texture_atlas,
            mip_level: 0,
            origin: Origin3d {
                x: 0,
                y: 0,
                z: texture_slice_index,
            },
            aspect: TextureAspect::All,
        },
        bytemuck::cast_slice(font_texture_slice),
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(bytes_per_row),
            rows_per_image: NonZeroU32::new(texture_slice_size.y),
        },
        wgpu::Extent3d {
            width: texture_slice_size.x,
            height: texture_slice_size.y,
            depth_or_array_layers: 1,
        },
    );

    Ok(FontCollection {
        fonts_characters: fonts_char_collection,
        characters_hasmap: font_collection_maps,
        char_texture_slices: font_collection_texture_slices,
    })
}