use bytemuck::cast_slice;
use wgpu::{BufferUsages, util::DeviceExt};

pub struct Rectangle {
	pub vertex_buffer: wgpu::Buffer,
	pub color_buffer: wgpu::Buffer,
	pub bindgroup: wgpu::BindGroup,
}

impl Rectangle {
	pub fn new(
		device: &wgpu::Device,
		bindgroup_layout: &wgpu::BindGroupLayout,
		color: [f32; 4],
		size: [f32; 2],
		position: [f32; 2],
	) -> Self {
		let width = size[0];
		let height = size[1];

		#[rustfmt::skip]
		let vertices = [
			[(-width / 2.0) + position[0], (-height / 2.0) + position[1]], // bottom left
			[( width / 2.0) + position[0], (-height / 2.0) + position[1]], // bottom right
			[( width / 2.0) + position[0], ( height / 2.0) + position[1]], // top right
			[(-width / 2.0) + position[0], (-height / 2.0) + position[1]], // bottom left
			[( width / 2.0) + position[0], ( height / 2.0) + position[1]], // top right
			[(-width / 2.0) + position[0], ( height / 2.0) + position[1]], // top left
		];

		let vertex_buffer: wgpu::Buffer =
			device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: Some("Vertex Buffer"),
				contents: cast_slice(&vertices),
				usage: wgpu::BufferUsages::VERTEX,
			});

		let color_buffer: wgpu::Buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Color Buffer"),
			contents: cast_slice(&color),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::VERTEX,
		});

		let bindgroup: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &bindgroup_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
					buffer: &color_buffer,
					offset: 0,
					size: None,
				}),
			}],
			label: Some("bind_group"),
		});

		Rectangle {
			vertex_buffer,
			color_buffer,
			bindgroup,
		}
	}

	pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
		wgpu::BindGroupEntry {
			binding,
			resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
				buffer: &self.color_buffer,
				offset: 0,
				size: None,
			}),
		}
	}
}

impl super::Shape for Rectangle {
	fn draw(&self, render_pass: &mut wgpu::RenderPass<'_>) {
		render_pass.set_bind_group(0, &self.bindgroup, &[]);
		render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
		render_pass.draw(0..6, 0..1);
	}
}
