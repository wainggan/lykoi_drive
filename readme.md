
# lykoi_drive

thin stateless graphics api over opengl (using `lykoi_gl`) for lykoi.

```rust,no_run
# let VERTICES = [0u8; 6];
use lykoi_drive::*;
use color::Color;

let shader = Shader::new_src(
	"#version 330 core
	
	out vec4 fragcolor;
	
	void main() {
		fragcolor = vec4(1.0);
	}",
	"#version 330 core

	layout (location = 0) in vec2 a_pos;

	void main() {
		gl_Position = vec4(a_pos.x, a_pos.y, 0.0, 1.0); 
	}",
).unwrap();

let model_format = VertexFormat::new()
	.field_vec2();
let model_buffer = VertexBuffer::new_from(&VERTICES, &model_format);

let mut render = Draw::new();

render.clear(None, Color::from_irgb(0x11, 0x11, 0x16));
render.draw(
	None,
	&model_buffer,
	&shader,
	UniformItem::new(),
	Config::new(),
);
```

