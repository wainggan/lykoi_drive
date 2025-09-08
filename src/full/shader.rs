
use std::marker::PhantomData;
use std::collections::HashMap;


pub struct ShaderPartFragment;
impl ShaderPartFragment {
	pub fn new(src: &str) -> Result<ShaderPart<Self>, String> {
		ShaderPart::new(src, lykoi_gl::ShaderType::FragmentShader)
	}
}

pub struct ShaderPartVertex;
impl ShaderPartVertex {
	pub fn new(src: &str) -> Result<ShaderPart<Self>, String> {
		ShaderPart::new(src, lykoi_gl::ShaderType::VertexShader)
	}
}

pub struct ShaderPart<T> {
	object: lykoi_gl::ShaderObject,
	kind: PhantomData<T>,
}
impl<T> ShaderPart<T> {
	fn new(src: &str, kind: lykoi_gl::ShaderType) -> Result<Self, String> {
		let object = lykoi_gl::create_shader(kind).unwrap();
		lykoi_gl::shader_source(&object, [src]);
		lykoi_gl::compile_shader(&object);

		if !lykoi_gl::get_shader_compile_status(&object) {
			let string = lykoi_gl::get_shader_info_log(&object);
			return Err(string);
		}

		Ok(Self {
			object,
			kind: PhantomData,
		})
	}
}
impl<T> Drop for ShaderPart<T> {
	fn drop(&mut self) {
		lykoi_gl::delete_shader(unsafe { std::ptr::read(&self.object) });
	}
}

pub struct Shader {
	object: lykoi_gl::ProgramObject,
	uniforms: HashMap<String, lykoi_gl::UniformTypes>,
}
impl Shader {
	pub fn new(
		vertex: ShaderPart<ShaderPartVertex>,
		fragment: ShaderPart<ShaderPartFragment>
	) -> Result<Self, String> {
		let object = lykoi_gl::create_program().unwrap();

		lykoi_gl::attach_shader(&object, &fragment.object);
		lykoi_gl::attach_shader(&object, &vertex.object);
		lykoi_gl::link_program(&object);

		if !lykoi_gl::get_program_link_status(&object) {
			let string = lykoi_gl::get_program_info_log(&object);
			return Err(string);
		}

		let amount = lykoi_gl::get_program_active_uniforms(&object);

		let mut uniforms = HashMap::new();

		for id in 0..amount {
			let (name, kind, _) = lykoi_gl::get_active_uniform(&object, id);
			uniforms.insert(name, kind);
		}

		Ok(Self {
			object,
			uniforms,
		})
	}
	pub fn new_src(vertex_src: &str, fragment_src: &str) -> Result<Self, String> {
		let shader_vs = Shader::new_vertex(vertex_src).unwrap();
		let shader_fs = Shader::new_fragment(fragment_src).unwrap();
		Self::new(shader_vs, shader_fs)
	}
	pub fn new_fragment(src: &str) -> Result<ShaderPart<ShaderPartFragment>, String> {
		ShaderPartFragment::new(src)
	}
	pub fn new_vertex(src: &str) -> Result<ShaderPart<ShaderPartVertex>, String> {
		ShaderPartVertex::new(src)
	}

	pub fn handle(&self) -> &lykoi_gl::ProgramObject {
		&self.object
	}
	pub fn get_uniform(&self, name: &str) -> Option<&lykoi_gl::UniformTypes> {
		self.uniforms.get(name)
	}
}
impl Drop for Shader {
	fn drop(&mut self) {
		lykoi_gl::delete_program(unsafe { std::ptr::read(&self.object) });
	}
}

