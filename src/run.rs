extern mod rsfml;
use rsfml::graphics::{Color, RenderWindow};
use rsfml::graphics::rc::{Sprite, Text};
use rsfml::system::{Vector2f, Vector2u};
mod control;
mod menu;
mod show;
mod window;

pub fn main_loop() {
	// Window declare and info retrieval
	let mut window = ::window::create();
	let window_vec:Vector2u = window.get_size();
	let (window_x, window_y) = (window_vec.x as f32, window_vec.y as f32);
	let (window_three_forth_x, window_half_y) = (window_x * 3.0 / 4.0, window_y / 2.0);
	// Main set of bools
	let (mut is_title, mut is_card_list) = (true, false);
	let (mut is_next_card, mut is_last_card) = (false, false);
	// Title Text
	let title_position = Vector2f::new(0.0, 0.0);
	let title_text = ::menu::new("../resources/font/Jura-DemiBold.ttf", "Welcome To Tarot",
	 30, &title_position);
	let directions_position = Vector2f::new(0.0, 35.0);
	let directions = ::menu::new("../resources/font/Jura-DemiBold.ttf", "Press Space To Continue",
		20, &directions_position);
	// What Card are we on
	let mut card_counter:int = 0;

	while window.is_open() {
		::control::exit(&mut window);
		// Title Screen
		if is_title{
			let (got_title, got_card_list) = ::control::menu();
			is_card_list = got_card_list;
			is_title = got_title;
			show_title(&mut window, &title_text, &directions);
		// Show all cards screen
		} else if is_card_list{
			if card_counter == -1 {card_counter = 77}
			if card_counter == 78 {card_counter = 0}

			let current_card = ::show::one(window_three_forth_x, window_half_y, card_counter);
			card_counter += ::control::card_shift(&mut window);

			show_all(&mut window, &current_card);
		}
		
	}
}

fn show_title(window: &mut RenderWindow, title_text: &Text, directions:&Text) {
	window.clear(&Color::white());
	window.draw(title_text); window.draw(directions);
	window.display()
}
fn show_all(window: &mut RenderWindow, current_card:&Sprite) {
	window.clear(&Color::white());
	window.draw(current_card);
	window.display()
}