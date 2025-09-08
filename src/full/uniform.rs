
use super::{Texture, TextureFormat};


pub enum UniformTypes<'a> {
	Float1(f32),
	Float2([f32; 2]),
	Float3([f32; 3]),
	Float4([f32; 4]),
	Mat4x4(&'a cgmath::Matrix4<f32>),
	Sampler2D(&'a Texture, &'a TextureFormat),
}

pub trait AsUniformType {
	fn convert(&'_ self) -> UniformTypes<'_>;
}

impl AsUniformType for () {
	fn convert(&'_ self) -> UniformTypes<'_> {
		unreachable!()
	}
}
impl AsUniformType for f32 {
	fn convert(&'_ self) -> UniformTypes<'_> {
		UniformTypes::Float1(*self)
	}
}
impl AsUniformType for [f32; 1] {
	fn convert(&'_ self) -> UniformTypes<'_> {
		UniformTypes::Float1(self[0])
	}
}
impl AsUniformType for [f32; 2] {
	fn convert(&'_ self) -> UniformTypes<'_> {
		UniformTypes::Float2(*self)
	}
}
impl AsUniformType for [f32; 3] {
	fn convert(&'_ self) -> UniformTypes<'_> {
		UniformTypes::Float3(*self)
	}
}
impl AsUniformType for [f32; 4] {
	fn convert(&'_ self) -> UniformTypes<'_> {
		UniformTypes::Float4(*self)
	}
}
impl AsUniformType for (&Texture, &TextureFormat) {
	fn convert(&'_ self) -> UniformTypes<'_> {
		UniformTypes::Sampler2D(self.0, self.1)
	}
}
impl AsUniformType for &cgmath::Matrix4<f32> {
	fn convert(&'_ self) -> UniformTypes<'_> {
		UniformTypes::Mat4x4(self)
	}
}

pub trait UniformList {
	fn visit(&self, callback: impl FnMut(&'static str, UniformTypes));
}

pub struct UniformEmpty;
impl UniformList for UniformEmpty {
	fn visit(&self, _: impl FnMut(&'static str, UniformTypes)) {
	}
}

pub struct UniformItem<T, N>
where T: AsUniformType, N: UniformList {
	name: Option<&'static str>,
	value: T,
	next: N,
}

impl<T, N> UniformList for UniformItem<T, N>
where T: AsUniformType, N: UniformList {
	fn visit(&self, mut callback: impl FnMut(&'static str, UniformTypes)) {
		if let Some(name) = self.name {
			callback(name, self.value.convert());
		}
		self.next.visit(callback);
	}
}

impl UniformItem<(), UniformEmpty> {
	pub fn new() -> Self {
		Self {
			name: None,
			value: (),
			next: UniformEmpty,
		}
	}
}

impl<T, N> UniformItem<T, N>
where T: AsUniformType, N: UniformList {
	pub fn add<U: AsUniformType>(
		self, name: &'static str, value: U
	) -> UniformItem<U, UniformItem<T, N>> {
		UniformItem {
			name: Some(name),
			value,
			next: self,
		}
	}
}

