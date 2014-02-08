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
	let (mut is_next_card, mut is_last_card) = (false, false);
	// Title Text
	let title_text = ::menu::new("../resources/font/Jura-DemiBold.ttf", "Welcome to Tarot", 30);
	// What Card are we on
	let mut card_counter:int = 1;

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
			if card_counter > 0 && card_counter < 5 {
				card_counter += ::control::card_shift(&mut window);
			}
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