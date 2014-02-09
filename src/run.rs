extern mod rsfml;
use rsfml::graphics::{Color, RenderWindow};
use rsfml::graphics::rc::{Sprite, Text};
use rsfml::system::{Vector2f, Vector2u};
use std::rand::{task_rng, Rng};
mod control;
mod deck;
mod menu;
mod window;

pub fn main_loop() {
	// Window declare and info retrieval
	let mut window = ::window::create();
	let window_vec:Vector2u = window.get_size();
	let (window_x, window_y) = (window_vec.x as f32, window_vec.y as f32);
	let (window_three_fourth_x, window_fourth_x, window_half_y, window_fourth_y) = (window_x * 3.0 / 4.0, window_x / 4.0, window_y / 2.0, window_y / 4.0);
	// Main set of bools
	let mut screen = 1; let mut is_set = false;
	// Title Text
	let title_text = ::menu::new("../resources/font/Jura-DemiBold.ttf", "Welcome To Tarot",
	 	30, window_fourth_x, window_fourth_y, 0.0);
	let directions_position = Vector2f::new(0.0, 35.0);
	let directions = ::menu::new("../resources/font/Jura-DemiBold.ttf", "Press Space To Continue",
		20, window_fourth_x, window_fourth_y, 35.0);
	// Find random card for title screen
	let mut rng = task_rng();
	let title_card = rng.gen_range(0, 77);
	// What Card are we on
	let mut card_counter = 1;
	let (mut card_one, mut card_two, mut card_three) = (0, 0, 0);
	let (mut all_cards, all_cards_desc) = ::deck::new(window_fourth_x, window_three_fourth_x, window_fourth_y, window_half_y);


	while window.is_open() {
		::control::exit(&mut window);

		match screen {
			// Title Screen
			1   =>	{
				let got_screen = ::control::menu();
				screen = got_screen;
				////////////////////////////////////////////////////////////////////////////////////
				// PART OF OLD METHOD //////////////////////////////////////////////////////////////
				//let current_card = ::show::one(window_three_forth_x, window_half_y, title_card);
				////////////////////////////////////////////////////////////////////////////////////
				show_title(&mut window, &title_text, &directions, &all_cards[title_card]);
			}
			// Show All Cards
			2	=>	{
				//////////////////////////////////////////////////////////////////////////////////////
				// OLD METHOD IN CASE I HAVE TO REVERT ///////////////////////////////////////////////
				//let current_card = ::show::one(window_three_forth_x, window_half_y, card_counter);
				//////////////////////////////////////////////////////////////////////////////////////
				let (got_counter, got_screen) = ::control::card_shift(&mut window);
				// Never go out of bounds
				if card_counter == 0 && got_counter == -1 {card_counter = 0}
					else if card_counter == 77 && got_counter == 1 {card_counter = 77} 
					else {card_counter += got_counter;} 
				
				screen = got_screen;
				is_set = false;
				show_all(&mut window, &all_cards[card_counter], &all_cards_desc[card_counter]);
			}
			// 3 Card Reading
			3	=>	{
				if is_set == false {
					// Generate three cards
					card_one = rng.gen_range(0, 77);
					card_two = rng.gen_range(0, 77);
					card_three = rng.gen_range(0, 77);
					// Set Position and Scale of each card
					&all_cards[card_one].set_position2f(window_x/6.0, window_y/2.0);&all_cards[card_one].set_scale2f(0.60, 0.60);
					&all_cards[card_two].set_position2f(window_x/2.0, window_y/2.0);&all_cards[card_two].set_scale2f(0.60, 0.60);
					&all_cards[card_three].set_position2f(window_x*5.0/6.0, window_y/2.0);&all_cards[card_three].set_scale2f(0.60, 0.60);

					is_set = true;
					show_reading(&mut window, &all_cards[card_one], &all_cards[card_two], &all_cards[card_three]);
				} else if is_set == true{
					let current_one = card_one;
					let current_two = card_two;
					let current_three = card_three;
					show_reading(&mut window, &all_cards[current_one], &all_cards[current_two], &all_cards[current_three]);
				}

				let got_screen = ::control::reading();
				screen = got_screen;
			}
			4	=> {
				
			}
			5	=> {
				show_blank(&mut window);
				screen = 1;
			}
			_		=>	{fail!(~"Error, could not figure out screen to be on.");}
		}
	}
}
fn show_blank(window: &mut RenderWindow) {
	window.clear(&Color::white());
	window.display()
}

fn show_title(window: &mut RenderWindow, title_text: &Text, directions:&Text, main_card:&Sprite) {
	window.clear(&Color::white());
	window.draw(title_text); window.draw(directions);
	window.draw(main_card);
	window.display()
}
fn show_all(window: &mut RenderWindow, current_card:&Sprite, current_description:&Text) {
	window.clear(&Color::white());
	window.draw(current_card); window.draw(current_description);
	window.display()
}
fn show_reading(window:&mut RenderWindow, first_card:&Sprite, second_card:&Sprite, third_card:&Sprite) {
	window.clear(&Color::white());
	window.draw(first_card); window.draw(second_card); window.draw(third_card);
	window.display()
}