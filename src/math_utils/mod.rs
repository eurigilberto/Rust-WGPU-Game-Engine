use glam::{Vec2, Vec3, Vec4};

macro_rules! impl_lerp {
	($name: ident,$num: ty) => {
		pub fn $name(a: $num, b: $num, t: $num)->$num{
			a + t * (b - a)
		}
	};
}

impl_lerp!(lerp_f32,f32);
impl_lerp!(lerp_f64,f64);

impl_lerp!(lerp_vec2,Vec2);
impl_lerp!(lerp_vec3,Vec3);
impl_lerp!(lerp_vec4,Vec4);