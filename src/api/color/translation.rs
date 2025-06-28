use super::{Color, ColorType};

#[const_trait]
pub trait ColorTranslation {
	fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> ColorType;
	fn from_hsla(hue: u16, saturation: u8, lightness: u8, alpha: f32) -> ColorType;
	fn from_hsva(hue: u16, saturation: u8, value: u8, alpha: f32) -> ColorType;
	fn from_cmyka(cyan: u8, magenta: u8, yellow: u8, key: u8, alpha: f32) -> ColorType;
	fn from_hwba(hue: u16, whiteness: f32, blackness: f32, alpha: f32) -> ColorType;
	fn from_hex(code: u32) -> ColorType;
}

impl const ColorTranslation for super::Color {
	fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> ColorType {
		[
			red as f32 / 255.0,
			green as f32 / 255.0,
			blue as f32 / 255.0,
			alpha as f32 / 255.0,
		]
	}

	fn from_hsla(hue: u16, saturation: u8, lightness: u8, alpha: f32) -> ColorType {
		let [saturation, lightness, alpha]: [f32; 3] =
			[saturation as f32 / 100.0, lightness as f32 / 100.0, alpha];

		let [chroma, hue_prime]: [f32; 2] = [
			(1.0 - (2.0 * lightness - 1.0)).abs() * saturation,
			hue as f32 / 60.0,
		];

		let [m, x]: [f32; 2] = [
			lightness - chroma / 2.0,
			chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs()),
		];

		let [r1, g1, b1] = match hue_prime as u32 {
			0 => [chroma, x, 0.0],
			1 => [x, chroma, 0.0],
			2 => [0.0, chroma, x],
			3 => [0.0, x, chroma],
			4 => [x, 0.0, chroma],
			5 => [chroma, 0.0, x],
			_ => [0.0, 0.0, 0.0],
		};

		[r1 + m, g1 + m, b1 + m, alpha]
	}

	fn from_hsva(hue: u16, saturation: u8, value: u8, alpha: f32) -> ColorType {
		let [saturation, value]: [f32; 2] = [saturation as f32 / 100.0, value as f32 / 100.0];
		let [hue, chroma]: [f32; 2] = [hue as f32 % 360.0, (saturation * value) as f32];
		let x = chroma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());

		let [r, g, b] = match hue {
			0.000..=60.00 => [chroma, x, 0.0],
			60.00..=120.0 => [x, chroma, 0.0],
			120.0..=180.0 => [0.0, chroma, x],
			180.0..=240.0 => [0.0, x, chroma],
			240.0..=300.0 => [x, 0.0, chroma],
			_ => [chroma, 0.0, x],
		};

		let match_value = value - chroma;

		[
			(r + match_value).clamp(0.0, 1.0),
			(g + match_value).clamp(0.0, 1.0),
			(b + match_value).clamp(0.0, 1.0),
			alpha,
		]
	}

	fn from_cmyka(cyan: u8, magenta: u8, yellow: u8, key: u8, alpha: f32) -> ColorType {
		const fn calculate(color: f32, key: f32) -> f32 {
			return 1.0 - (color * (1.0 - key) + key);
		}

		const fn check_if_in_range(color: u8) -> f32 {
			return (color as f32 / 255.0).clamp(0.0, 1.0);
		}

		let [cyan, magenta, yellow, key] = [
			check_if_in_range(cyan),
			check_if_in_range(magenta),
			check_if_in_range(yellow),
			check_if_in_range(key),
		];

		[
			calculate(cyan, key),
			calculate(magenta, key),
			calculate(yellow, key),
			alpha,
		]
	}

	/// # assertions
	/// 0 >= hue >= 360 \
	/// 0 >= whiteness >= 100 \
	/// 0 >= blackness >= 100 \
	/// 0 >= alpha >= 1
	fn from_hwba(hue: u16, whiteness: f32, blackness: f32, alpha: f32) -> ColorType {
		assert!(
			hue <= 360
				&& whiteness <= 100.
				&& whiteness >= 0.
				&& blackness <= 100.
				&& blackness >= 0.
				&& alpha >= 0.
				&& alpha <= 1.
		);

		let [whiteness, blackness, alpha]: [f32; 3] = [whiteness / 100.0, blackness / 100.0, alpha];
		let [hue_prime, chroma]: [f32; 2] = [hue as f32 / 60.0, 1.0 - whiteness - blackness];

		let x = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());

		let rgb = match hue_prime {
			0.0..=1.0 => [chroma, x, 0.0],
			1.0..=2.0 => [x, chroma, 0.0],
			2.0..=3.0 => [0.0, chroma, x],
			3.0..=4.0 => [0.0, x, chroma],
			4.0..=5.0 => [x, 0.0, chroma],
			_ => [chroma, 0.0, x],
		};

		let mut result = [0_f32; 3];
		let mut index: usize = 0;

		while index < 3 {
			let color = rgb[index];

			result[index] = (color + whiteness) * (1.0 - blackness);
			index += 1;
		}

		[result[0], result[1], result[2], alpha]
	}

	fn from_hex(code: u32) -> ColorType {
		const SHIFT: u32 = 8;

		const fn calculate(code: u32, places: u32) -> f32 {
			return ((code >> places) & 0xFF) as f32 / 255.0;
		}

		[
			calculate(code, SHIFT * 3),
			calculate(code, SHIFT * 2),
			calculate(code, SHIFT * 1),
			calculate(code, SHIFT * 0),
		]
	}
}
