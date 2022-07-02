use glam::{Vec2, Vec3, Vec4};

macro_rules! impl_lerp {
    ($name: ident,$num: ty) => {
        pub fn $name(a: $num, b: $num, t: $num) -> $num {
            a + t * (b - a)
        }
    };
}

impl_lerp!(lerp_f32, f32);
impl_lerp!(lerp_f64, f64);

impl_lerp!(lerp_vec2, Vec2);
impl_lerp!(lerp_vec3, Vec3);
impl_lerp!(lerp_vec4, Vec4);

pub fn easeOutBack(t: f32) -> f32 {
    const c1: f32 = 1.70158;
    const c3: f32 = c1 + 1.0;

    return 1.0 + c3 * f32::powf(t - 1.0, 3.0) + c1 * f32::powf(t - 1.0, 2.0);
}
pub fn easeInBack(t: f32) -> f32 {
    const c1: f32 = 1.70158;
    const c3: f32 = c1 + 1.0;

    return c3 * t * t * t - c1 * t * t;
}
