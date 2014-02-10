extern mod rsfml;
use rsfml::window::{ContextSettings, VideoMode, Close};
use rsfml::graphics::{RenderWindow};
use rsfml::system::{Vector2u};

pub fn create(window_size:Vector2u) -> RenderWindow{
	let window_width = window_size.x as uint; let window_height= window_size.y as uint;
	let setting = ContextSettings::default();
	let mut window = match RenderWindow::new(VideoMode::new_init(window_width, window_height, 32), "Tarot", Close, &setting) {
		Some(window)	=>	window,
		None()			=>	fail!(~"Error, creating window"),
	};
	window.set_key_repeat_enabled(false);
	window
}