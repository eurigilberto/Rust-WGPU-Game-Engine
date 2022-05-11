use crate::color::hsla::HSLA;

pub enum RGBElems{
	R,
	G,
	B
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RGBA{
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32
}

impl RGBA {
	pub fn new(r:f32, g:f32, b:f32, a:f32) -> Self{
		Self{
			r,g,b,a
		}
	}
	pub fn rrr1(r: f32) -> Self{
		Self{
			r,g:r,b:r,a:1.0
		}
	}
	pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
	pub const GREY: Self = Self {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
}

impl From<RGBA> for [f32;4]{
	fn from(rgba: RGBA)->Self{
		return [rgba.r, rgba.g, rgba.b, rgba.a]
	}
}

impl From<RGBA> for HSLA{
	fn from(rgba: RGBA)->Self{
		let mut cmax = (rgba.r, RGBElems::R);
		if cmax.0 < rgba.g {
			cmax = (rgba.g, RGBElems::G);
		}
		if cmax.0 < rgba.b {
			cmax = (rgba.b, RGBElems::B);
		}

		let cmin = f32::min(f32::min(rgba.r,rgba.g), rgba.b);
		let delta = cmax.0 - cmin;
		let mut hue;
		let lightness = (cmax.0 - cmin) / 2.0;
		let mut sat = 0.0;
		if f32::abs(delta) > f32::EPSILON {
			sat = delta / (1.0 - f32::abs(2.0 * lightness - 1.0));
		}
		let c_div_delta = |a:f32,b:f32| (a - b) / delta;

		match cmax.1 {
			RGBElems::R => {
				hue = 60.0 * (c_div_delta(rgba.g, rgba.b).rem_euclid(6.0));
			}
			RGBElems::G => {
				hue = 60.0 * (c_div_delta(rgba.b, rgba.r) + 2.0);
			}
			RGBElems::B => {
				hue = 60.0 * (c_div_delta(rgba.r, rgba.g) + 4.0);
			}
		};

		Self{
			h: hue,
			s: sat,
			l: lightness,
			a: rgba.a
		}
	}
}