#![allow(unused)]
mod presets;
mod translation;

pub use {presets::*, translation::*};

pub type ColorType = [f32; 4];
pub struct Color;
