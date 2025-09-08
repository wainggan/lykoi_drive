
use std::ptr::slice_from_raw_parts;

use cgmath::Matrix;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum ConfigBlend {
	Zero,
	One,
	SrcColor,
	InvSrcColor,
	DstColor,
	InvDstColor,
	SrcAlpha,
	InvSrcAlpha,
	DstAlpha,
	InvDstAlpha,
	ConstColor,
	InvConstColor,
	ConstAlpha,
	InvConstAlpha,
}
impl ConfigBlend {
	fn to(&self) -> u32 {
		match self {
			ConfigBlend::Zero => gl::ZERO,
			ConfigBlend::One => gl::ONE,
			ConfigBlend::SrcColor => gl::SRC_COLOR,
			ConfigBlend::InvSrcColor => gl::ONE_MINUS_SRC_COLOR,
			ConfigBlend::DstColor => gl::DST_COLOR,
			ConfigBlend::InvDstColor => gl::ONE_MINUS_DST_COLOR,
			ConfigBlend::SrcAlpha => gl::SRC_ALPHA,
			ConfigBlend::InvSrcAlpha => gl::ONE_MINUS_SRC_ALPHA,
			ConfigBlend::DstAlpha => gl::DST_ALPHA,
			ConfigBlend::InvDstAlpha => gl::ONE_MINUS_DST_ALPHA,
			ConfigBlend::ConstColor => gl::CONSTANT_COLOR,
			ConfigBlend::InvConstColor => gl::ONE_MINUS_CONSTANT_COLOR,
			ConfigBlend::ConstAlpha => gl::CONSTANT_ALPHA,
			ConfigBlend::InvConstAlpha => gl::ONE_MINUS_CONSTANT_ALPHA,
		}
	}
}

#[derive(Debug, Clone)]
pub struct Config {
	test_depth: bool,
	blendmode: Option<([ConfigBlend; 2], Option<[ConfigBlend; 2]>)>,
	blendcolor: color::Color,
}
impl Config {
	pub fn new() -> Self {
		Self {
			test_depth: false,
			blendmode: None,
			blendcolor: color::Color::from_frgba(1.0, 1.0, 1.0, 1.0),
		}
	}

	pub fn depth(mut self, test: bool) -> Self {
		self.test_depth = test;
		return self;
	}

	pub fn blend(mut self, src: ConfigBlend, dst: ConfigBlend) -> Self {
		self.blendmode = Some(([src, dst], None));
		return self;
	}
	pub fn blend_sep(
		mut self,
		src_c: ConfigBlend,
		dst_c: ConfigBlend,
		src_a: ConfigBlend,
		dst_a: ConfigBlend
	) -> Self {
		self.blendmode = Some(([src_c, dst_c], Some([src_a, dst_a])));
		return self;
	}

	pub fn blend_color(mut self, color: color::Color) -> Self {
		self.blendcolor = color;
		return self;
	}
}


pub struct Draw {
}
impl Draw {
	pub fn new() -> Self {
		Self {
		}
	}

	pub fn clear(&mut self, target: Option<&Surface>, color: color::Color) {
		if let Some(surf) = target {
			lykoi_gl::viewport(0, 0, surf.width(), surf.height());
			lykoi_gl::bind_framebuffer(lykoi_gl::FramebufferTarget::Framebuffer, surf.handle_framebuffer());
		} else {
			lykoi_gl::viewport(0, 0, 640, 480);
			lykoi_gl::unbind_framebuffer(lykoi_gl::FramebufferTarget::Framebuffer);
		}

		lykoi_gl::clear_color(color.r, color.g, color.b, color.a);
		lykoi_gl::clear(&[lykoi_gl::BufferBit::ColorBufferBit, lykoi_gl::BufferBit::DepthBufferBit]);

		if let Some(_) = target {
			lykoi_gl::viewport(0, 0, 640, 480);
			lykoi_gl::unbind_framebuffer(lykoi_gl::FramebufferTarget::Framebuffer);
		}
	}

	pub fn draw<T>(
		&mut self,
		target: Option<&Surface>,
		vertex: &VertexBuffer<T>,
		shader: &Shader,
		uniform: impl UniformList,
		config: Config,
	) {
		
		if let Some(surf) = target {
			lykoi_gl::viewport(0, 0, surf.width(), surf.height());
			lykoi_gl::bind_framebuffer(lykoi_gl::FramebufferTarget::Framebuffer, surf.handle_framebuffer());
		} else {
			lykoi_gl::viewport(0, 0, 640, 480);
			lykoi_gl::unbind_framebuffer(lykoi_gl::FramebufferTarget::Framebuffer);
		}
		lykoi_gl::use_program(&shader.handle());

		let mut i_tex = 0u32;

		uniform.visit(|name, value| {
			let Some(kind) = shader.get_uniform(name) else {
				panic!("uniform name '{}' doesn't exist", name);
			};

			let Some(object) = lykoi_gl::get_uniform_location(
				&shader.handle(), name,
			) else {
				panic!("uniform name '{}' not found", name);
			};

			match (kind, value) {
				(lykoi_gl::UniformTypes::Float, UniformTypes::Float1(x))
					=> lykoi_gl::uniform_1f(&object, x),

				(lykoi_gl::UniformTypes::FloatVec2, UniformTypes::Float2([x, y]))
					=> lykoi_gl::uniform_2f(&object, x, y),

				(lykoi_gl::UniformTypes::FloatVec3, UniformTypes::Float3([x, y, z]))
					=> lykoi_gl::uniform_3f(&object, x, y, z),

				(lykoi_gl::UniformTypes::FloatVec3, UniformTypes::Float4([x, y, z, w]))
					=> lykoi_gl::uniform_4f(&object, x, y, z, w),
				
				(_, UniformTypes::Mat4x4(v))
					=> lykoi_gl::uniform_matrix_4fv(
						&object,
						false,
						unsafe {
							&*slice_from_raw_parts(v.as_ptr() as *const f32, 16)
						}
					),

				(lykoi_gl::UniformTypes::Sampler2D, UniformTypes::Sampler2D(t, f))
					=> {
						lykoi_gl::uniform_1i(&object, i_tex as i32);
						lykoi_gl::active_texture(lykoi_gl::raw::TEXTURE0 + i_tex);

						lykoi_gl::bind_texture(lykoi_gl::BindTextureTarget::Texture2D, &t.handle());

						let value_wrap = match f.wrap {
							TextureFormatWrap::Repeat => lykoi_gl::TexParameterWrap::Repeat,
							TextureFormatWrap::Clamp => lykoi_gl::TexParameterWrap::ClampToEdge,
							TextureFormatWrap::Border => lykoi_gl::TexParameterWrap::ClampToBorder,
						};
						lykoi_gl::tex_parameter_wrap_s(lykoi_gl::TexParameterTarget::Texture2D, value_wrap);
						lykoi_gl::tex_parameter_wrap_t(lykoi_gl::TexParameterTarget::Texture2D, value_wrap);

						let value_filter_mag = match f.filter {
							TextureFormatFilter::Linear => lykoi_gl::TexParameterMagFilter::Linear,
							TextureFormatFilter::Nearest => lykoi_gl::TexParameterMagFilter::Nearest,
						};
						let value_filter_min = match f.filter {
							TextureFormatFilter::Linear => lykoi_gl::TexParameterMinFilter::Linear,
							TextureFormatFilter::Nearest => lykoi_gl::TexParameterMinFilter::Nearest,
						};
						lykoi_gl::tex_parameter_mag_filter(lykoi_gl::TexParameterTarget::Texture2D, value_filter_mag);
						lykoi_gl::tex_parameter_min_filter(lykoi_gl::TexParameterTarget::Texture2D, value_filter_min);
						
						i_tex += 1;
					},

				_ => todo!(),
			}

		});

		{
			if config.test_depth {
				lykoi_gl::enable(lykoi_gl::raw::DEPTH_TEST);
			} else {
				lykoi_gl::disable(lykoi_gl::raw::DEPTH_TEST);
			}

			if let Some(clr) = config.blendmode {
				lykoi_gl::enable(lykoi_gl::raw::BLEND);

				if let (clr, Some(alp)) = clr {
					lykoi_gl::blend_func_seperate(clr[0].to(), clr[1].to(), alp[0].to(), alp[1].to());
				} else {
					let clr = clr.0;
					lykoi_gl::blend_func(clr[0].to(), clr[1].to());
				}

			} else {
				lykoi_gl::disable(gl::BLEND);
			}

			lykoi_gl::blend_color(
				config.blendcolor.r,
				config.blendcolor.g,
				config.blendcolor.b,
				config.blendcolor.a,
			);
		}

		lykoi_gl::bind_vertex_array(vertex.handle_array());
		lykoi_gl::draw_arrays(lykoi_gl::DrawPrimitives::Triangles, 0, vertex.get_verts_len() as u32);
		lykoi_gl::unbind_vertex_array();

		if let Some(_) = target {
			lykoi_gl::viewport(0, 0, 640, 480);
			lykoi_gl::unbind_framebuffer(lykoi_gl::FramebufferTarget::Framebuffer);
		}

	}

}

