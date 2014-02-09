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
	let (mut card_one_rand, mut card_two_rand, mut card_three_rand, mut card_four_rand, mut card_five_rand) = (0,0,0,0,0);
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
				is_set = false;
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
				show_all(&mut window, &all_cards[card_counter], &all_cards_desc[card_counter]);
			}
			// 3 Card Reading
			3	=>	{
				if is_set == false {
					// Generate three cards
					card_one_rand = rng.gen_range(0, 77);
					card_two_rand = rng.gen_range(0, 77);
					if card_one_rand != card_two_rand {} else {card_two_rand = rng.gen_range(0, 77);}
					card_three_rand = rng.gen_range(0, 77);
					if (card_three_rand != card_two_rand) || (card_three_rand != card_one_rand) {} else {card_three_rand = rng.gen_range(0, 77);}
					is_set = true;
					// Set Position and Scale of each card
					
				} else if is_set == true{
					let mut card_one = match all_cards[card_one_rand].clone() {
						Some(card_one)	=>	card_one,
						None()			=>	fail!(~"Error, card_one created.")
					};
					let mut card_two = match all_cards[card_two_rand].clone() {
						Some(card_two)	=>	card_two,
						None()			=>	fail!(~"Error, card_two created.")
					};
					let mut card_three = match all_cards[card_three_rand].clone() {
						Some(card_three)	=>	card_three,
						None()			=>	fail!(~"Error, card_three created.")
					};
					&card_one.set_position2f(window_x/6.0, window_y/3.0);&card_one.set_scale2f(0.60, 0.60);
					&card_two.set_position2f(window_x/2.0, window_y*2.0/3.0);&card_two.set_scale2f(0.60, 0.60);
					&card_three.set_position2f(window_x*5.0/6.0, window_y/3.0);&card_three.set_scale2f(0.60, 0.60);
					let mut cards = ~[card_one, card_two, card_three];
					cards.shrink_to_fit();
					show_reading(&mut window, cards);
				}

				// Check to switch screens
				let got_screen = ::control::reading(3);
				screen = got_screen;
			}
			// 5 Card Spread
			4	=> {
				if is_set == false {
					card_one_rand = rng.gen_range(0, 77);
					card_two_rand = rng.gen_range(0, 77);
					if card_one_rand != card_two_rand {} else {card_two_rand = rng.gen_range(0, 77);}
					card_three_rand = rng.gen_range(0, 77);
					if (card_three_rand != card_two_rand) || (card_three_rand != card_one_rand) {} else {card_three_rand = rng.gen_range(0, 77);}
					card_four_rand = rng.gen_range(0, 77);
					if (card_four_rand != card_two_rand) || (card_four_rand != card_one_rand) || (card_four_rand != card_three_rand) {} else {card_three_rand = rng.gen_range(0, 77);}
					card_five_rand = rng.gen_range(0, 77);
					if (card_five_rand != card_two_rand) || (card_five_rand != card_one_rand) || (card_five_rand != card_four_rand) || (card_five_rand != card_three_rand) {} else {card_three_rand = rng.gen_range(0, 77);}
					is_set = true;
				} else if is_set == true{
					let mut card_one = match all_cards[card_one_rand].clone() {
						Some(card_one)	=>	card_one,
						None()			=>	fail!(~"Error, card_one created.")
					};
					let mut card_two = match all_cards[card_two_rand].clone() {
						Some(card_two)	=>	card_two,
						None()			=>	fail!(~"Error, card_two created.")
					};
					let mut card_three = match all_cards[card_three_rand].clone() {
						Some(card_three)	=>	card_three,
						None()			=>	fail!(~"Error, card_three created.")
					};
					let mut card_four = match all_cards[card_four_rand].clone() {
						Some(card_four)	=>	card_four,
						None()			=>	fail!(~"Error, card_four created.")
					};
					let mut card_five = match all_cards[card_five_rand].clone() {
						Some(card_five)	=>	card_five,
						None()			=>	fail!(~"Error, card_five created.")
					};

					&card_one.set_position2f(window_x/6.0, window_y/4.0);&card_one.set_scale2f(0.40, 0.40);
					&card_two.set_position2f(window_x/3.0, window_y*3.0/4.0);&card_two.set_scale2f(0.40, 0.40);
					&card_three.set_position2f(window_x/2.0, window_y/4.0);&card_three.set_scale2f(0.40, 0.40);
					&card_four.set_position2f(window_x*4.0/6.0, window_y*3.0/4.0);&card_four.set_scale2f(0.40, 0.40);
					&card_five.set_position2f(window_x*5.0/6.0, window_y/4.0);&card_five.set_scale2f(0.40, 0.40);
					let mut cards = ~[card_one, card_two, card_three, card_four, card_five];
					cards.shrink_to_fit();

					let got_screen = ::control::reading(4);
					screen = got_screen;
					show_reading(&mut window, cards);
				}
				
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
fn show_reading(window:&mut RenderWindow, cards:~[Sprite]) {
	window.clear(&Color::white());
	for card_layout in cards.iter() {
		window.draw(card_layout);
	}
	window.display()
}
