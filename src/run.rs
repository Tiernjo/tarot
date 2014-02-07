extern mod rsfml;
use rsfml::graphics::{Color, RenderWindow};
mod control;
mod window;

pub fn main_loop() {
	let mut window = ::window::create();
	while window.is_open() {
		::control::exit(&mut window);
		show(&mut window);
	}
}

fn show(window: &mut RenderWindow) {
	window.clear(&Color::white());
	window.display()
}