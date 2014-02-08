extern mod rsfml;
use rsfml::graphics::{Color, RenderWindow, Text};
use rsfml::system::{Vector2u};
mod control;
mod menu;
mod window;

pub fn main_loop() {
	// Window declare and info retrieval
	let mut window = ::window::create();
	let window_vec:Vector2u = window.get_size();
	// Main set of bools
	let (mut is_title, mut is_card_list) = (true, false);

	while window.is_open() {
		::control::exit(&mut window);
		// Title Screen
		if is_title{
			let title_text = ::menu::new();
			let (got_title, got_card_list) = ::control::menu(&mut window);
			is_card_list = got_card_list;
			is_title = got_title;
			show_title(&mut window, title_text);
		// Show all cards screen
		} else if is_card_list{
			show_all(&mut window);
		}
		
	}
}

fn show_title(window: &mut RenderWindow, title_text: Text) {
	window.clear(&Color::white());
	window.draw(title_text);
	window.display()
}
fn show_all(window: &mut RenderWindow) {
	window.clear(&Color::black());
	window.display()
}