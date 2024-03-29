use glam::{uvec2, UVec2};
use half::f16;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sdf_glyph_renderer::{render_sdf, BitmapGlyph};

#[derive(Clone)]
pub struct FontTextureSlice {
    ///Glyph that represents this
    pub character: char,
    ///Top left coordinate for the glyph on the texture
    pub tex_coord: UVec2,
    pub buffer_size: usize,
    ///Font Metrics
    pub metrics: fontdue::Metrics,
}
impl Default for FontTextureSlice {
    fn default() -> Self {
        Self {
            character: Default::default(),
            tex_coord: Default::default(),
            buffer_size: Default::default(),
            metrics: Default::default(),
        }
    }
}
impl FontTextureSlice {
    pub fn get_padded_size(&self) -> UVec2 {
        uvec2(self.get_padded_width(), self.get_padded_height())
    }
    pub fn get_padded_width(&self) -> u32 {
        (self.metrics.width + self.buffer_size * 2) as u32
    }
    pub fn get_padded_height(&self) -> u32 {
        (self.metrics.height + self.buffer_size * 2) as u32
    }
}

#[derive(Debug)]
pub enum FontCreationError {
    FileRead(std::io::Error),
    FontFileParsing(String),
    NotEnoughSpaceOnTexture,
}

pub struct FontAtlas {
    pub font_glyphs: Vec<FontTextureSlice>,
    pub font_sdf_texture: Vec<f16>,
}

pub fn parse_font_from_bytes(font_bytes: &[u8], scale: f32) -> fontdue::FontResult<fontdue::Font> {
    fontdue::Font::from_bytes(
        font_bytes,
        fontdue::FontSettings {
            scale,
            ..Default::default()
        },
    )
}

#[derive(Clone, Copy)]
pub enum FontCharLimit {
    All,
    LimitedCount(usize),
}

macro_rules! print_time {
    ( $x:expr , $msg: expr ) => {{
        let time = std::time::Instant::now();
        let res = $x;
        println!(
            "{} - elapsed time {} millis",
            $msg,
            time.elapsed().as_millis()
        );
        res
    }};
}

impl FontAtlas {
    pub fn new(
        file_data: &[u8],
        reqested_size: UVec2,
        character_size: f32,
        font_char_limit: FontCharLimit,
    ) -> Result<Self, FontCreationError> {
        match parse_font_from_bytes(file_data, character_size) {
            Ok(font) => {
                let mut slice_coords = print_time!(
                    create_character_slices(&font, character_size, 8, font_char_limit),
                    "Character slice generated"
                );
                let bitmaps = print_time!(
                    create_character_bitmaps(&font, &slice_coords, character_size, 8),
                    "Character bitmap generated"
                );
                let bitmaps_sdf_half = print_time!(
                    generate_sdf_bitmaps(&bitmaps, 8),
                    "Character sdf bitmaps generated"
                );
                let res_font_sdf_texture = print_time!(
                    create_font_sdf_texture(&mut slice_coords, &bitmaps_sdf_half, reqested_size),
                    "Font sdf texture"
                );
                match res_font_sdf_texture {
                    Ok(font_sdf_texture) => {
                        return Ok(Self {
                            font_glyphs: slice_coords,
                            font_sdf_texture: font_sdf_texture,
                        });
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            Err(error) => return Err(FontCreationError::FontFileParsing(String::from(error))),
        }
    }
}

fn create_character_slices(
    font: &fontdue::Font,
    character_size: f32,
    buffer_size: usize,
    font_char_limit: FontCharLimit,
) -> Vec<FontTextureSlice> {
    let mut char_array: Vec<char> = font.chars().keys().into_iter().map(|key| *key).collect();
    char_array.sort();
    let selected_chars = match font_char_limit {
        FontCharLimit::All => char_array.clone(),
        FontCharLimit::LimitedCount(count) => {
            let mut s_char = Vec::<char>::with_capacity(count);
            for index in 0..count {
                s_char.push(char_array[index]);
            }
            s_char
        }
    };

    let character_count = selected_chars.len();
    let mut slice_coords = vec![
        FontTextureSlice {
            ..Default::default()
        };
        character_count
    ];
    for (index, char_ref) in selected_chars.iter().enumerate() {
        let character = *char_ref;
        let char_index = font.lookup_glyph_index(character);

        let metrics = font.metrics_indexed(char_index, character_size);
        slice_coords[index].character = character;
        slice_coords[index].metrics = metrics;
        slice_coords[index].buffer_size = buffer_size;
    }
    slice_coords.sort_by(|a, b| a.metrics.height.cmp(&b.metrics.height));
    //let something: Vec<i32> = slice_coords.iter().map(|slice| {print!(" {:?} ", slice.character); 0}).collect();
    //Slices sorted by character height to pack them more tightly on the texture

    slice_coords
}

fn create_character_bitmaps(
    font: &fontdue::Font,
    slice_coords: &Vec<FontTextureSlice>,
    character_size: f32,
    buffer_size: usize,
) -> Vec<BitmapGlyph> {
    let mut bitmaps: Vec<BitmapGlyph> = Vec::<BitmapGlyph>::with_capacity(slice_coords.len());
    for (index, slice) in slice_coords.iter().enumerate() {
        //Get character bitmap
        let char_index = font.lookup_glyph_index(slice.character);

        //Add glyph data to texture
        font.metrics_indexed(char_index, character_size);
        let (_, bitmap) = font.rasterize_indexed(char_index, character_size);

        //Create intermidiate character bitmap with buffer zone
        let padded_width = slice.metrics.width + buffer_size * 2;
        let padded_height = slice.metrics.height + buffer_size * 2;
        let mut padded_bitmap = vec![0; padded_width * padded_height];

        for v_index in 0..slice.metrics.height {
            let bitmap_slice =
                &bitmap[v_index * slice.metrics.width..(v_index + 1) * slice.metrics.width];
            for (h_index, value) in bitmap_slice.iter().enumerate() {
                let tex_h_coord = buffer_size + h_index;
                let tex_v_coord = buffer_size + v_index;
                let tex_index = tex_h_coord + tex_v_coord * padded_width;
                padded_bitmap[tex_index] = *value;
            }
        }

        bitmaps.push(BitmapGlyph::new(
            padded_bitmap,
            slice.metrics.width,
            slice.metrics.height,
            buffer_size,
        ));
    }
    bitmaps
}

pub fn generate_sdf_bitmaps(bitmaps: &Vec<BitmapGlyph>, padding_size: usize) -> Vec<Vec<f16>> {
    bitmaps
        .par_iter()
        .map(|bitmap| {
            let bitmap_sdf = render_sdf(bitmap, padding_size);
            let bitmap_sdf_half: Vec<f16> = bitmap_sdf
                .iter()
                .map(|value| f16::from_f32(*value as f32))
                .collect();
            bitmap_sdf_half
        })
        .collect()
}

fn create_font_sdf_texture(
    slice_coords: &mut Vec<FontTextureSlice>,
    sdf_bitmaps: &Vec<Vec<f16>>,
    reqested_size: UVec2,
) -> Result<Vec<f16>, FontCreationError> {
    let mut cursor = uvec2(0, 0);
    let mut line_height = 0;

    let texture_size = (reqested_size.x * reqested_size.y) as usize;
    let mut font_texture_data: Vec<f16> = vec![f16::from_f32(128.0); texture_size];

    for (index, slice) in slice_coords.iter_mut().enumerate() {
        let fits_horizontally = (cursor.x + slice.get_padded_width()) < reqested_size.x;
        let fits_vertically = (cursor.y + slice.get_padded_height()) < reqested_size.y;

        if !fits_vertically && !fits_horizontally {
            return Err(FontCreationError::NotEnoughSpaceOnTexture);
        }

        if !fits_horizontally {
            cursor.x = 0;
            cursor.y += line_height + 4;
            line_height = 0;
        }

        let fits_vertically = (cursor.y + slice.get_padded_height()) < reqested_size.y;
        if !fits_vertically {
            println!("Cursor: {:?}", cursor);
            return Err(FontCreationError::NotEnoughSpaceOnTexture);
        }

        let sdf_bitmap = &sdf_bitmaps[index];

        let char_sdf_size = sdf_bitmap.len();
        let expected_sdf_size = (slice.get_padded_width() * slice.get_padded_height()) as usize;

        assert_eq!(char_sdf_size, expected_sdf_size, "SDF has incorrect size");

        let start_position = cursor.clone();
        let end_position = uvec2(
            cursor.x + slice.get_padded_width(),
            cursor.y + slice.get_padded_height(),
        );

        let slc_padded_width = slice.get_padded_width() as usize;
        let slc_padded_height = slice.get_padded_height() as usize;
        for v_index in 0..slc_padded_height {
            let start_index = v_index * slc_padded_width;
            let end_index = (v_index + 1) * slc_padded_width;
            let line_range = start_index .. end_index;
            for (h_index, value_index) in line_range.enumerate() {
                let value = sdf_bitmap[value_index];

                let tex_h_coord = cursor.x + h_index as u32;
                let tex_v_coord = cursor.y + v_index as u32;

                let pixel_position = uvec2(tex_h_coord, tex_v_coord);
                let inside_expected_region = start_position.x <= pixel_position.x
                    && start_position.y <= pixel_position.y
                    && end_position.x >= pixel_position.x
                    && end_position.y >= pixel_position.y;
                
                assert!(
                    inside_expected_region,
                    "Not inside expected region {:?}",
                    pixel_position
                );

                let tex_index = (tex_h_coord + tex_v_coord * reqested_size.x) as usize;

                font_texture_data[tex_index] = value;
            }
        }

        slice.tex_coord = cursor.clone();

        line_height = if line_height < slice.get_padded_height() {
            slice.get_padded_height()
        } else {
            line_height
        };
        cursor.x += slice.get_padded_width();
    }
    Ok(font_texture_data)
}
