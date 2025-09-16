
use std::cell::OnceCell;

#[derive(Debug, Clone, Copy)]
pub enum TextureFormatWrap {
	Repeat,
	Clamp,
	Border,
}

#[derive(Debug, Clone, Copy)]
pub enum TextureFormatFilter {
	Linear,
	Nearest,
}

#[derive(Debug, Clone)]
pub struct TextureFormat {
	pub wrap: TextureFormatWrap,
	pub filter: TextureFormatFilter,
}
impl TextureFormat {
	pub fn new() -> Self {
		Self {
			wrap: TextureFormatWrap::Clamp,
			filter: TextureFormatFilter::Linear,
		}
	}

	pub fn set_wrap(mut self, option: TextureFormatWrap) -> Self {
		self.wrap = option;
		self
	}

	pub fn set_filter(mut self, option: TextureFormatFilter) -> Self {
		self.filter = option;
		self
	}
}


#[derive(Debug)]
pub struct Texture {
	size: (usize, usize),
	object: lykoi_gl::TextureObject,
	surface: OnceCell<Surface>,
}
impl Texture {
	pub fn new(width: usize, height: usize, data: &[u8]) -> Self {
		if data.len() != (width * height * 4) as usize {
			panic!("expected size {}, got {}", data.len(), width * height * 4);
		}

		let [object] = lykoi_gl::gen_textures();

		lykoi_gl::bind_texture(lykoi_gl::BindTextureTarget::Texture2D, &object);
		lykoi_gl::tex_image_2d(
			lykoi_gl::TexImage2DTarget::Texture2D,
			0,
			lykoi_gl::TexImageInnerFormat::RGBA,
			width,
			height,
			lykoi_gl::TexImageDataFormat::RGBA,
			lykoi_gl::TexImageDataType::UnsignedByte,
			Some(data),
		);
		lykoi_gl::unbind_texture(lykoi_gl::BindTextureTarget::Texture2D);

		Self {
			object,
			size: (width, height),
			surface: OnceCell::new(),
		}
	}
	pub fn new_empty(width: usize, height: usize) -> Self {
		let [object] = lykoi_gl::gen_textures();

		lykoi_gl::bind_texture(lykoi_gl::BindTextureTarget::Texture2D, &object);
		lykoi_gl::tex_image_2d(
			lykoi_gl::TexImage2DTarget::Texture2D,
			0,
			lykoi_gl::TexImageInnerFormat::RGBA,
			width,
			height,
			lykoi_gl::TexImageDataFormat::RGBA,
			lykoi_gl::TexImageDataType::UnsignedByte,
			None,
		);
		lykoi_gl::unbind_texture(lykoi_gl::BindTextureTarget::Texture2D);

		Self {
			object,
			size: (width, height),
			surface: OnceCell::new(),
		}
	}

	pub fn handle(&self) -> &lykoi_gl::TextureObject {
		&self.object
	}
	pub fn width(&self) -> usize {
		self.size.0
	}
	pub fn height(&self) -> usize {
		self.size.1
	}

	pub fn surface(&self) -> &Surface {
		match self.surface.get() {
			Some(v) => v,
			None => {
				let surf = Surface::new(self);
				self.surface.set(surf).unwrap();
				self.surface.get().unwrap()
			},
		}
	}
}

#[derive(Debug)]
pub struct Surface {
	object_framebuffer: lykoi_gl::FramebufferObject,
	object_renderbuffer: lykoi_gl::RenderbufferObject,
	size: (usize, usize),
}
impl Surface {
	fn new(target: &Texture) -> Self {
		let [object_framebuffer] = lykoi_gl::gen_framebuffers();
		lykoi_gl::bind_framebuffer(lykoi_gl::FramebufferTarget::Framebuffer, &object_framebuffer);

		lykoi_gl::framebuffer_texture_2d(
			lykoi_gl::FramebufferTarget::Framebuffer,
			lykoi_gl::FramebufferAttachment::ColorAttachment,
			gl::TEXTURE_2D,
			target.handle(),
		);

		let [object_renderbuffer] = lykoi_gl::gen_renderbuffers();
		lykoi_gl::bind_renderbuffer(&object_renderbuffer);
		lykoi_gl::renderbuffer_storage(gl::DEPTH24_STENCIL8, target.width(), target.height());
		lykoi_gl::unbind_renderbuffer();

		lykoi_gl::framebuffer_renderbuffer(
			lykoi_gl::FramebufferTarget::Framebuffer,
			gl::DEPTH_STENCIL_ATTACHMENT,
			&object_renderbuffer
		);

		if !lykoi_gl::check_framebuffer_status(lykoi_gl::FramebufferTarget::Framebuffer) {
			panic!("oops");
		}

		lykoi_gl::unbind_framebuffer(lykoi_gl::FramebufferTarget::Framebuffer);

		Self {
			object_framebuffer,
			object_renderbuffer,
			size: target.size,
		}
	}

	pub fn handle_framebuffer(&self) -> &lykoi_gl::FramebufferObject {
		&self.object_framebuffer
	}
	pub fn handle_renderbuffer(&self) -> &lykoi_gl::RenderbufferObject {
		&self.object_renderbuffer
	}

	pub fn width(&self) -> usize {
		self.size.0
	}
	pub fn height(&self) -> usize {
		self.size.1
	}
}

