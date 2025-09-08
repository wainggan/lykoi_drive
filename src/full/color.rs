
#[derive(Debug, Clone, Copy)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}
impl Color {
	pub fn from_irgb(r: u8, g: u8, b: u8) -> Self {
		Self {
			r: r as f32 / 255.0,
			g: g as f32 / 255.0,
			b: b as f32 / 255.0,
			a: 1.0,
		}
	}
	pub fn from_irgba(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self {
			r: r as f32 / 255.0,
			g: g as f32 / 255.0,
			b: b as f32 / 255.0,
			a: a as f32 / 255.0,
		}
	}
	pub fn from_frgb(r: f32, g: f32, b: f32) -> Self {
		Self {
			r,
			g,
			b,
			a: 1.0,
		}
	}
	pub fn from_frgba(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self {
			r,
			g,
			b,
			a,
		}
	}
	pub fn from_hex(_hex: u32) -> Self {
		todo!()
	}
	pub fn from_hex_str(hex: &str) -> Option<Self> {
		let hex = hex.trim_start_matches("#");
		match hex.len() {
			3 => {
				let (r, hex) = hex.split_at(1);
				let (g, b) = hex.split_at(1);
				let Ok(r)= u8::from_str_radix(r, 16) else {
					return None;
				};
				let Ok(g)= u8::from_str_radix(g, 16) else {
					return None;
				};
				let Ok(b)= u8::from_str_radix(b, 16) else {
					return None;
				};
				let r = r as f32 * 17.0 / 255.0;
				let g = g as f32 * 17.0 / 255.0;
				let b = b as f32 * 17.0 / 255.0;
				Some(Self::from_frgb(r, g, b))
			},
			4 => {
				let (r, hex) = hex.split_at(1);
				let (g, hex) = hex.split_at(1);
				let (b, a) = hex.split_at(1);
				let Ok(r)= u8::from_str_radix(r, 16) else {
					return None;
				};
				let Ok(g)= u8::from_str_radix(g, 16) else {
					return None;
				};
				let Ok(b)= u8::from_str_radix(b, 16) else {
					return None;
				};
				let Ok(a)= u8::from_str_radix(a, 16) else {
					return None;
				};
				let r = r as f32 * 17.0 / 255.0;
				let g = g as f32 * 17.0 / 255.0;
				let b = b as f32 * 17.0 / 255.0;
				let a = a as f32 * 17.0 / 255.0;
				Some(Self::from_frgba(r, g, b, a))
			},
			6 => {
				let (r, hex) = hex.split_at(2);
				let (g, b) = hex.split_at(2);
				let Ok(r)= u8::from_str_radix(r, 16) else {
					return None;
				};
				let Ok(g)= u8::from_str_radix(g, 16) else {
					return None;
				};
				let Ok(b)= u8::from_str_radix(b, 16) else {
					return None;
				};
				let r = r as f32 / 255.0;
				let g = g as f32 / 255.0;
				let b = b as f32 / 255.0;
				Some(Self::from_frgb(r, g, b))
			},
			8 => {
				let (r, hex) = hex.split_at(2);
				let (g, hex) = hex.split_at(2);
				let (b, a) = hex.split_at(2);
				let Ok(r)= u8::from_str_radix(r, 16) else {
					return None;
				};
				let Ok(g)= u8::from_str_radix(g, 16) else {
					return None;
				};
				let Ok(b)= u8::from_str_radix(b, 16) else {
					return None;
				};
				let Ok(a)= u8::from_str_radix(a, 16) else {
					return None;
				};
				let r = r as f32 / 255.0;
				let g = g as f32 / 255.0;
				let b = b as f32 / 255.0;
				let a = a as f32 / 255.0;
				Some(Self::from_frgba(r, g, b, a))
			},
			_ => None
		}
	}
}

