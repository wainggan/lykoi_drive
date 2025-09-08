
use crate::{Config, Shader, Surface, Texture, UniformList, VertexBuffer, VertexFormat};

pub trait Vertex2D {
	fn make2d(pos: (f32, f32), tex: (f32, f32)) -> Self;
	fn get_format() -> VertexFormat;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
	pub pos: [f32; 2],
	pub tex: [f32; 2],
}
impl Vertex2D for Vertex {
	fn make2d(pos: (f32, f32), tex: (f32, f32)) -> Self {
		Self {
			pos: pos.into(),
			tex: tex.into(),
		}
	}
	fn get_format() -> VertexFormat {
		VertexFormat::new()
			.field_vec2()
			.field_vec2()
	}
}

/**
vertex buffer abstraction.

`Sprite<T>` is a container that automatically constructs a vertex buffer around a texture.

the vertex buffer is refreshed with [`Self::reframe()`]. a vertex format definition is 
required to do so, however, which is given with `Sprite`'s `T` (see: [`Vertex2D`]). the format
[`Vertex`] is already provided, and a `Sprite<Vertex>` can be created using [`Self::new_simple()`].
in a shader, location 0 will be the position, and location 1 will be the uvs.

if a custom format `T: Vertex2D` is required, however, you can create the
`Sprite<T>` using [`Self::new_via()`].
*/
pub struct Sprite<'a, T = Vertex> 
where T: Clone + Vertex2D {
	texture: &'a Texture,
	buffer: VertexBuffer<T>,
	offset: (u32, u32),
	size: (u32, u32),
	verts: Vec<T>,
}
impl<'a> Sprite<'a> {
	/**
	creates a new `Sprite<Vertex>`.
	 */
	pub fn new_simple(from: &'a Texture, offset: (u32, u32), size: (u32, u32)) -> Self {
		Self::new_via(from, offset, size)
	}
}
impl<'a, T> Sprite<'a, T>
where T: Clone + Vertex2D {
	fn new_via(from: &'a Texture, offset: (u32, u32), size: (u32, u32)) -> Self {
		Self {
			texture: from,
			buffer: VertexBuffer::new(&T::get_format()),
			offset,
			size,
			verts: Vec::new(),
		}
	}

	pub fn reframe(&mut self, frame: u8) {
		self.verts.clear();

		let ox = self.offset.0 + frame as u32 * self.size.0;
		let oy = self.offset.1;

		let px0 = ox as f32 / self.texture.width() as f32;
		let py0 = oy as f32 / self.texture.height() as f32;
		let px1 = (ox + self.size.0) as f32 / self.texture.width() as f32;
		let py1 = (oy + self.size.1) as f32 / self.texture.height() as f32;

		self.verts.extend_from_slice(&[
			T::make2d(
				(0.0, 1.0),
				(px0, py1)
			),
			T::make2d(
				(1.0, 0.0),
				(px1, py0),
			),
			T::make2d(
				(0.0, 0.0),
				(px0, py0),
			),

			T::make2d(
				(0.0, 1.0),
				(px0, py1),
			),
			T::make2d(
				(1.0, 1.0),
				(px1, py1),
			),
			T::make2d(
				(1.0, 0.0),
				(px1, py0),
			),
		]);

		self.buffer.rebuffer(&self.verts);
	}
}


pub trait Techniques {
	fn draw_sprite<T: Clone + Vertex2D>(
		&mut self,
		target: Option<&Surface>,
		sprite: &Sprite<T>,
		shader: &Shader,
		uniform: impl UniformList,
		config: Config,
	);
}
impl Techniques for crate::Draw {
	fn draw_sprite<T: Clone + Vertex2D>(
		&mut self,
		target: Option<&crate::Surface>,
		sprite: &Sprite<T>,
		shader: &Shader,
		uniform: impl UniformList,
		config: Config,
	) {
		self.draw(
			target,
			&sprite.buffer,
			shader,
			uniform,
			config,
		);
	}
}


