pub fn lerp(a: &f32, b: &f32, step: &f32) -> f32 {
	a + (b-a) * step
}

pub fn sin(f: f32) -> f32 {
	f.sin()
}
