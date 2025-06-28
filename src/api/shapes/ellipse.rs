use bytemuck::cast_slice;
use wgpu::{BufferUsages, util::DeviceExt};

pub struct Ellipse {
	geometry: VertexBuffer,
	appearance: Appearance,
}

struct VertexBuffer {
	pub buffer: wgpu::Buffer,
	pub vertex_amount: u32,
	pub segment_amount: u32,
}

struct Appearance {
	pub color_buffer: wgpu::Buffer,
	pub bind_group: wgpu::BindGroup,
}

impl Ellipse {
	pub const DEFAULT_AMOUNT_OF_SEGMENTS: u32 = 64;

	pub fn new(
		device: &wgpu::Device,
		bind_group_layout: &wgpu::BindGroupLayout,
		color: [f32; 4],
		size: [f32; 2],
		position: [f32; 2],
		segments: u32,
	) -> Self {
		assert!(size[0] >= 0.0 || size[1] >= 0.0 && segments > 3);
		let vertices = Self::generate_vertices(position, size, segments);

		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: None,
			contents: cast_slice(&vertices),
			usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
		});

		let color_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: None,
			contents: cast_slice(&color),
			usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: bind_group_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
					buffer: &color_buffer,
					offset: 0,
					size: None,
				}),
			}],
			label: None,
		});

		Self {
			geometry: VertexBuffer {
				buffer: vertex_buffer,
				vertex_amount: vertices.len() as u32,
				segment_amount: segments,
			},
			appearance: Appearance {
				color_buffer,
				bind_group,
			},
		}
	}

	fn generate_vertices(center: [f32; 2], size: [f32; 2], segments: u32) -> Vec<[f32; 2]> {
		let mut vertices = Vec::with_capacity((segments * 3) as usize);

		for i in 0..segments {
			let theta0 = (i as f32 / segments as f32) * std::f32::consts::TAU;
			let theta1 = ((i + 1) % segments) as f32 / segments as f32 * std::f32::consts::TAU;

			let (x0, y0) = (
				center[0] + size[0] * theta0.cos(),
				center[1] + size[1] * theta0.sin(),
			);

			let (x1, y1) = (
				center[0] + size[0] * theta1.cos(),
				center[1] + size[1] * theta1.sin(),
			);

			vertices.push(center);
			vertices.push([x0, y0]);
			vertices.push([x1, y1]);
		}

		vertices
	}

	pub fn set_color(&self, queue: &wgpu::Queue, color: [f32; 4]) {
		queue.write_buffer(&self.appearance.color_buffer, 0, cast_slice(&color));
	}

	pub fn set_size(&self, queue: &wgpu::Queue, position: [f32; 2], size: [f32; 2]) {
		let vertices = Self::generate_vertices(position, size, self.geometry.segment_amount);
		queue.write_buffer(&self.geometry.buffer, 0, cast_slice(&vertices));
	}

	pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
		wgpu::BindGroupEntry {
			binding,
			resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
				buffer: &self.appearance.color_buffer,
				offset: 0,
				size: None,
			}),
		}
	}
}
impl super::Shape for Ellipse {
	fn draw(&self, render_pass: &mut wgpu::RenderPass<'_>) {
		render_pass.set_bind_group(0, &self.appearance.bind_group, &[]);
		render_pass.set_vertex_buffer(0, self.geometry.buffer.slice(..));
		render_pass.draw(0..self.geometry.vertex_amount, 0..1);
	}
}
