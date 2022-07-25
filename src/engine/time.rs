use std::ops::{Add, AddAssign, Deref, Mul, Sub};

macro_rules! time_unit_add {
    ($type:ident) => {
        impl Add for $type {
            type Output = $type;

            fn add(self, rhs: Self) -> Self::Output {
                $type(self.0 + rhs.0)
            }
        }
        impl AddAssign for $type {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }
    };
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Microsecond(pub u128);
impl Microsecond {
    pub fn as_millisecond(&self) -> Millisecond {
        let millis = self.0 / 1000;
        let millis_rem = ((self.0 % 1000) as f32) / 1000.0;
        Millisecond(millis as f32 + millis_rem)
    }

    pub fn as_seconds(&self) -> Second {
        let seconds = self.0 / 1000000;
        let seconds_rem = ((self.0 % 1000000) as f32) / 1000000.0;
        Second(seconds as f32 + seconds_rem)
    }
}
time_unit_add!(Microsecond);
#[derive(Default, Clone, Copy)]
pub struct FrameNumber(pub u128);
time_unit_add!(FrameNumber);

#[repr(C)]
#[derive(Default, Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Millisecond(pub f32);

#[repr(C)]
#[derive(Default, Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Second(pub f32);
impl Sub for Second{
    type Output = Second;

    fn sub(self, rhs: Self) -> Self::Output {
        Second(self.0 - rhs.0)
    }
}

impl Mul<f32> for Second{
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}