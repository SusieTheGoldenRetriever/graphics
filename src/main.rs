use {
	graphics::api::*,
	std::sync::Arc,
	winit::{
		application::ApplicationHandler,
		event::{ElementState, KeyEvent, WindowEvent},
		event_loop::ActiveEventLoop,
		keyboard::{Key, NamedKey},
		window::{Icon, Window, WindowId},
	},
};

const ICON: &[u8] = include_bytes!("../assets/icon-256*256.rgba");

#[derive(Default)]
pub struct App {
	graphics: Option<Graphics<'static>>,
}

impl ApplicationHandler for App {
	fn resumed(&mut self, active_eventloop: &ActiveEventLoop) {
		if !self.graphics.is_none() {
			return;
		}

		// window's icon doesn't appear

		let window_icon =
			Icon::from_rgba(ICON.to_vec(), 256, 256).expect("failed to get icon from rgba");
		let window_attributes = Window::default_attributes()
			.with_transparent(true)
			.with_window_icon(Some(window_icon));

		let message = "failed to create window";

		let window = active_eventloop
			.create_window(window_attributes)
			.expect(message);

		self.graphics = Some(Graphics::new(Arc::new(window)))
	}

	fn window_event(
		&mut self,
		active_eventloop: &ActiveEventLoop,
		_window_id: WindowId,
		event: WindowEvent,
	) {
		if self.graphics.is_none() {
			return;
		}

		let graphics = &mut self.graphics.as_mut().unwrap();
		let window = &graphics.window.as_ref();

		window.set_title("UNTITLED WINDOW");
		window.set_resizable(true);
		graphics.configure();

		graphics
			.shape_manager
			.rectangle(color::Color::WHITE, [1.0, 1.0], [0.0, 0.0]);

		graphics
			.shape_manager
			.rectangle([0.0, 1.0, 0.0, 0.4], [0.5, 1.0], [0.0, 0.0]);

		/*graphics.insert(Rectangle (
			[0.0, 1.0, 0.0, 0.4],
			[0.5, 1.0],
			[0.0, 0.0],
		));*/

		graphics
			.shape_manager
			.ellipse([0.0, 1.0, 0.0, 0.4], [0.25, 0.5], [0.0, 0.0]);

		event_handling(event, active_eventloop, graphics);
	}
}

fn event_handling(
	event: WindowEvent,
	active_eventloop: &ActiveEventLoop,
	graphics: &mut Graphics<'static>,
) {
	use WindowEvent::*;

	match event {
		Touch(touch) => println!("{:?}", touch.location),
		Resized(new_size) => println!("{:?}", new_size),
		CloseRequested => active_eventloop.exit(),
		RedrawRequested => graphics.redraw(),

		KeyboardInput {
			event:
				KeyEvent {
					repeat: false,
					state: ElementState::Pressed,
					logical_key: Key::Named(NamedKey::Escape),
					..
				},
			..
		} => active_eventloop.exit(),

		_ => (),
	}
}

/// Without this transparency doesn't work on wayland, or at least wayfire.
/// Wasn't tested on other compositors or desktop environments.
pub fn ensure_wayland_support() {
	use std::env::{remove_var, var};
	let wayland_display: &str = "WAYLAND_DISPLAY";

	if var(wayland_display).is_ok() {
		unsafe {
			remove_var(wayland_display);
		}
	}
}

fn main() {
	ensure_wayland_support();
	eventloop(App::default());
}
