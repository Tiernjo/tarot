extern mod rsfml;
use rsfml::graphics::{Color, RenderWindow};
use rsfml::graphics::rc::{Sprite, Text};
use rsfml::system::{Vector2u};
mod control;
mod menu;
mod show;
mod window;

pub fn main_loop() {
	// Window declare and info retrieval
	let mut window = ::window::create();
	let window_vec:Vector2u = window.get_size();
	// Main set of bools
	let (mut is_title, mut is_card_list) = (true, false);
	// Title Text
	let title_text = ::menu::new("../resources/font/Jura-DemiBold.ttf", "Welcome to Tarot", 30);
	// What Card are we on
	let card_counter:uint = 0;

	while window.is_open() {
		::control::exit(&mut window);
		// Title Screen
		if is_title{
			let (got_title, got_card_list) = ::control::menu();
			is_card_list = got_card_list;
			is_title = got_title;
			show_title(&mut window, &title_text);
		// Show all cards screen
		} else if is_card_list{
			let current_card = ::show::one(card_counter);
			show_all(&mut window, &current_card);
		}
		
	}
}

fn show_title(window: &mut RenderWindow, title_text: &Text) {
	window.clear(&Color::white());
	window.draw(title_text);
	window.display()
}
fn show_all(window: &mut RenderWindow, current_card:&Sprite) {
	window.clear(&Color::black());
	window.draw(current_card);
	window.display()
}