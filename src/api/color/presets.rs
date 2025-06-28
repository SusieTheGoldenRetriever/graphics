use super::{Color, ColorTranslation};

macro_rules! generate_color_presets {
  ($($name:ident: $hex:expr),* $(,)?) => {
    $(pub const $name: [f32; 4] = Self::from_hex($hex);)*
  };
}

impl Color {
	generate_color_presets! {
		RED: 0xff0000ff,
		GREEN: 0x00ff00ff,
		BLUE: 0x0000ffff,
		YELLOW: 0xffff00ff,
		CYAN: 0x00ffffff,
		MAGENTA: 0xff00ffff,
		BLACK: 0x000000ff,
		WHITE: 0xffffffff,
		GRAY: 0x808080ff,
		PURPLE: 0x800080ff,
		PINK: 0xffc0cbff,
		ORANGE: 0xffa500ff,
		BROWN: 0xa52a2aff,
		BEIGE: 0xf5f5dcff,
		VIOLET: 0xee82eeff,
		INDIGO: 0x4b0082ff,
		SCARLET: 0xff2400ff,
		CRIMSON: 0xdc143cff,
		MAROON: 0x800000ff,
		OLIVE: 0x808000ff,
		LIME: 0x00ff00ff,
		TEAL: 0x008080ff,
		AQUA: 0x00ffffff,
		SILVER: 0xc0c0c0ff,
		GOLD: 0xffd700ff,
		IVORY: 0xfffff0ff,
	}
}
