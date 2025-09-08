
pub struct VertexBuffer<T> {
	prepared: bool,
	object_buffer: lykoi_gl::BufferObject,
	object_array: lykoi_gl::VertexArrayObject,
	data: Option<Vec<T>>,
	format: VertexFormat,
}
impl<T> VertexBuffer<T> {
	pub fn new(format: &VertexFormat) -> Self {
		let [object_buffer] = lykoi_gl::gen_buffers();
		let [object_array] = lykoi_gl::gen_vertex_arrays();

		lykoi_gl::bind_vertex_array(&object_array);

		lykoi_gl::bind_buffer(lykoi_gl::BindBufferTarget::ArrayBuffer, &object_buffer);
		format.emit();

		lykoi_gl::unbind_vertex_array();

		Self {
			prepared: false,
			object_buffer,
			object_array,
			data: None,
			format: format.clone(),
		}
	}

	pub fn handle_buffer(&self) -> &lykoi_gl::BufferObject {
		&self.object_buffer
	}
	pub fn handle_array(&self) -> &lykoi_gl::VertexArrayObject {
		&self.object_array
	}
	pub fn get_verts_len(&self) -> usize {
		if let Some(ref data) = self.data {
			data.len()
		} else {
			0
		}
	}
}

impl<T: Clone> VertexBuffer<T> {
	pub fn new_from(data: &[T], format: &VertexFormat) -> Self {
		let mut s = Self::new(format);
		s.rebuffer(data);
		return s;
	}

	pub fn rebuffer(&mut self, data: &[T]) {
		lykoi_gl::bind_vertex_array(&self.object_array);

		lykoi_gl::bind_buffer(lykoi_gl::BindBufferTarget::ArrayBuffer, &self.object_buffer);
		lykoi_gl::buffer_data(lykoi_gl::BindBufferTarget::ArrayBuffer, &data, lykoi_gl::BufferDataUsage::StaticDraw);

		lykoi_gl::unbind_vertex_array();
		
		self.prepared = true;
		self.data = Some(data.to_vec());
	}
}


#[derive(Debug, Clone)]
pub struct VertexFormat {
	fields: Vec<(u8, u8)>,
	stride: u32,
}
impl VertexFormat {
	pub fn new() -> Self {
		Self {
			fields: Vec::new(),
			stride: 0,
		}
	}

	pub fn bound(self) -> Self {
		self
	}

	fn field(&mut self, len: u8) {
		let size = size_of::<f32>();
		self.stride += size as u32 * len as u32;
		self.fields.push((len, size as u8));
	}

	pub fn field_vec4(mut self) -> Self {
		self.field(4);
		self
	}

	pub fn field_vec3(mut self) -> Self {
		self.field(3);
		self
	}

	pub fn field_vec2(mut self) -> Self {
		self.field(2);
		self
	}

	pub fn field_float(mut self) -> Self {
		self.field(1);
		self
	}

	fn emit(&self) {
		let mut offset = 0;
		for i in 0..self.fields.len() {
			let f = &self.fields[i];
			lykoi_gl::vertex_attrib_pointer(
				i as u32,
				f.0,
				lykoi_gl::VertexAttribPointerType::Float,
				false,
				self.stride,
				offset,
			);
			lykoi_gl::enable_vertex_attrib_array(i as u32);
			offset += f.0 as u32 * f.1 as u32;
		}
	}
}

