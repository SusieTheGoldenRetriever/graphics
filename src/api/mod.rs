pub mod color;
pub mod shapes;
pub use color::{Color, ColorTranslation};

use {
	shapes::*,
	std::{borrow::Cow, sync::Arc},
	wgpu::{ShaderSource, TextureFormat},
	winit::{
		event_loop::{ControlFlow, EventLoop},
		window::Window,
	},
};

pub struct Graphics<'window> {
	pub _instance: wgpu::Instance,
	pub adapter: wgpu::Adapter,
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
	pub render_pipeline: wgpu::RenderPipeline,
	pub surface: wgpu::Surface<'window>,
	pub window: std::sync::Arc<Window>,
	pub shape_manager: ShapeManager,
	pub bindgroup_layout: wgpu::BindGroupLayout,
}

impl<'window> Graphics<'window> {
	pub async fn create_async(window: Arc<Window>) -> Self {
		let _instance = wgpu::Instance::default();
		let surface = _instance
			.create_surface(Arc::clone(&window))
			.expect("failed to create surface");
		let adapter = _instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),
				force_fallback_adapter: false,
				compatible_surface: Some(&surface),
			})
			.await
			.expect("Failed to find an appropriate adapter");

		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					label: None,
					required_features: wgpu::Features::empty(),
					required_limits: wgpu::Limits {
						max_storage_buffers_per_shader_stage: 1,
						..wgpu::Limits::downlevel_webgl2_defaults()
					},
					memory_hints: wgpu::MemoryHints::Performance,
				},
				None,
			)
			.await
			.expect("Failed to create device");

		surface.configure(&device, &{
			let size = window.inner_size();

			surface
				.get_default_config(&adapter, size.width.max(1), size.height.max(1))
				.expect("failed to get default config for surface")
		});

		let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("Color BindGroupLayout"),
			entries: &[wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: None,
				},
				count: None,
			}],
		});

		let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: None,
			source: { ShaderSource::Wgsl(Cow::Borrowed(include_str!("assets/shader.wgsl"))) },
		});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: None,
			layout: Some(
				&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
					label: None,
					bind_group_layouts: &[&bindgroup_layout],
					push_constant_ranges: &[],
				}),
			),
			vertex: wgpu::VertexState {
				module: &shader_module,
				entry_point: Some("vs_main"),
				buffers: &[wgpu::VertexBufferLayout {
					array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
					step_mode: wgpu::VertexStepMode::Vertex,
					attributes: &wgpu::vertex_attr_array![0 => Float32x2],
				}],
				compilation_options: Default::default(),
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader_module,
				entry_point: Some("fs_main"),
				compilation_options: Default::default(),
				targets: &[Some(wgpu::ColorTargetState {
					format: TextureFormat::Bgra8UnormSrgb,
					blend: Some(wgpu::BlendState {
						// Enable alpha blending
						color: wgpu::BlendComponent {
							src_factor: wgpu::BlendFactor::SrcAlpha,
							dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
							operation: wgpu::BlendOperation::Add,
						},
						alpha: wgpu::BlendComponent {
							src_factor: wgpu::BlendFactor::One,
							dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
							operation: wgpu::BlendOperation::Add,
						},
					}),
					write_mask: wgpu::ColorWrites::ALL,
				})],
			}),
			primitive: wgpu::PrimitiveState::default(),
			depth_stencil: None,
			multisample: wgpu::MultisampleState::default(),
			multiview: None,
			cache: None,
		});

		/*let shape = Triangle::new(
			&device,
			&[0.0, 1.0, 0.0, 1.0],
			&[[0.0, 0.5], [-0.5, -0.5], [0.5, -0.5]],
		);*/

		Graphics {
			shape_manager: ShapeManager::new(device.clone(), bindgroup_layout.clone()),
			bindgroup_layout,
			adapter,
			device,
			queue,
			render_pipeline,
			surface,
			window,
			_instance,
		}
	}

	pub fn new(window: Arc<Window>) -> Self {
		pollster::block_on(Self::create_async(window))
	}

	pub fn window_dimensions(&self) -> winit::dpi::PhysicalSize<u32> {
		self.window.inner_size()
	}

	pub fn configure(&self) {
		let dimensions = self.window_dimensions();

		self.surface.configure(
			&self.device,
			&self
				.surface
				.get_default_config(&self.adapter, dimensions.width, dimensions.height)
				.unwrap(),
		);
	}

	pub fn render(&self, encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView) {
		let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			label: None,
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view: &texture_view,
				resolve_target: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
					store: wgpu::StoreOp::Store,
				},
			})],
			depth_stencil_attachment: None,
			timestamp_writes: None,
			occlusion_query_set: None,
		});

		render_pass.set_pipeline(&self.render_pipeline);

		for shape in self.shape_manager.shapes.iter() {
			shape.draw(&mut render_pass);
		}
	}

	pub fn redraw(&self) {
		let (frame, texture_view) = {
			let frame = self
				.surface
				.get_current_texture()
				.expect("Failed to acquire next swap chain texture");

			let texture_view = frame
				.texture
				.create_view(&wgpu::TextureViewDescriptor::default());

			(frame, texture_view)
		};

		let mut encoder = self
			.device
			.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

		self.render(&mut encoder, &texture_view);
		self.queue.submit(Some(encoder.finish()));
		frame.present();
	}
}

pub fn eventloop(mut app: impl winit::application::ApplicationHandler) {
	let eventloop = EventLoop::new().expect("failed to create eventloop");

	eventloop.set_control_flow(ControlFlow::Poll);
	eventloop
		.run_app(&mut app)
		.expect("failed to run application in eventloop");
}
