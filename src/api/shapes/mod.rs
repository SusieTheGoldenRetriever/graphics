#![allow(unused)]
pub mod ellipse;
pub mod manager;
pub mod rectangle;
pub mod triangle;

pub use {ellipse::Ellipse, manager::ShapeManager, rectangle::Rectangle, triangle::Triangle};

pub trait Shape {
	fn draw(&self, render_pass: &mut wgpu::RenderPass<'_>);
}
