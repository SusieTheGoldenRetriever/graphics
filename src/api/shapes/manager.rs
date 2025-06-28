use super::*;

pub struct ShapeManager {
	pub shapes: Vec<Box<dyn Shape>>,
	pub device: wgpu::Device,
	pub bind_group_layout: wgpu::BindGroupLayout,
}

impl ShapeManager {
	pub fn new(device: wgpu::Device, bind_group_layout: wgpu::BindGroupLayout) -> Self {
		ShapeManager {
			shapes: vec![],
			device,
			bind_group_layout,
		}
	}

	pub fn ellipse(&mut self, color: [f32; 4], size: [f32; 2], position: [f32; 2]) {
		self.shapes.push(Box::new(Ellipse::new(
			&self.device,
			&self.bind_group_layout,
			color,
			size,
			position,
			64,
		)));
	}

	pub fn rectangle(&mut self, color: [f32; 4], size: [f32; 2], position: [f32; 2]) {
		self.shapes.push(Box::new(Rectangle::new(
			&self.device,
			&self.bind_group_layout,
			color,
			size,
			position,
		)));
	}

	pub fn triangle(&mut self, color: [f32; 4], vertices: [[f32; 2]; 3]) {
		self.shapes.push(Box::new(Triangle::new(
			&self.device,
			&self.bind_group_layout,
			color,
			vertices,
		)));
	}
}
