use std::ops::{Add, Mul, Sub};

pub fn lerp<T>(a: T, b: T, t: f32) -> <T as Add>::Output
where
	T: Copy + Sub<Output = T> + Mul<f32, Output = T> + Add<T, Output = T>,
{
	a + (b - a) * t
}