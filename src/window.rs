extern mod rsfml;
use rsfml::window::{ContextSettings, VideoMode, Close};
use rsfml::graphics::{RenderWindow};

pub fn create() -> RenderWindow{
	let setting = ContextSettings::default();
	let window = match RenderWindow::new(VideoMode::new_init(900, 600, 32), "Tarot", Close, &setting) {
		Some(window)	=>	window,
		None()			=>	fail!(~"Error, creating window"),
	};
	window
}