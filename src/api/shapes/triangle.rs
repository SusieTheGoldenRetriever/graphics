use bytemuck::cast_slice;
use wgpu::{BufferUsages, util::DeviceExt};

pub struct Triangle {
	pub vertex_buffer: wgpu::Buffer,
	pub color_buffer: wgpu::Buffer,
	pub bindgroup: wgpu::BindGroup,
}

impl Triangle {
	pub fn new(
		device: &wgpu::Device,
		bindgroup_layout: &wgpu::BindGroupLayout,
		color: [f32; 4],
		vertices: [[f32; 2]; 3],
	) -> Self {
		let vertex_buffer: wgpu::Buffer = device.create_buffer_init(&{
			wgpu::util::BufferInitDescriptor {
				label: Some("Vertex Buffer"),
				contents: cast_slice(&vertices),
				usage: BufferUsages::UNIFORM | BufferUsages::VERTEX,
			}
		});

		let color_buffer: wgpu::Buffer = device.create_buffer_init(&{
			wgpu::util::BufferInitDescriptor {
				label: Some("Vertex Buffer"),
				contents: cast_slice(&color),
				usage: BufferUsages::UNIFORM | BufferUsages::VERTEX,
			}
		});

		let bindgroup: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &bindgroup_layout, // The layout defined earlier
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

		Triangle {
			vertex_buffer,
			color_buffer,
			bindgroup,
		}
	}
}

impl super::Shape for Triangle {
	fn draw(&self, render_pass: &mut wgpu::RenderPass<'_>) {
		render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
		render_pass.draw(0..3, 0..1);
	}
}
