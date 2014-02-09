extern mod rsfml;
use rsfml::graphics::{RenderWindow, Texture};
use rsfml::graphics::rc::{Sprite, Text};
use rsfml::system::{Vector2f};
use rsfml::system::{Vector2f};
use std::cell::RefCell;
use std::rc::Rc;

struct Card<'s>{
	name:&'s str,
	img_loc:&'s str,
	str_loc:&'s str,
}
impl <'s>Card<'s> {
	fn texture(&self, file:&str) -> Texture{
		let texture = match Texture::new_from_file(file) {
			Some(texture)	=>	texture,
			None()			=>	fail!(~"Error, card block")
		};
		texture
	}
	fn sprite(&self) -> Sprite {
		let sprite = match Sprite::new() {
			Some(sprite)	=>	sprite,
			None()			=>	fail!(~"Error, card sprite"),
		};
		sprite
	}
}

pub fn new(window_three_forth_x:f32, window_half_y:f32) -> ~[Sprite] {
	let mut major_arcana = major_arcana(window_three_forth_x, window_half_y);
	let mut cups = cups(window_three_forth_x, window_half_y);
	let mut pentacles = pentacles(window_three_forth_x, window_half_y);
	let mut wands = wands(window_three_forth_x, window_half_y);
	let mut swords = swords(window_three_forth_x, window_half_y);

	let mut all = ~[];
	all.push_all_move(major_arcana);
	all.push_all_move(cups);
	all.push_all_move(pentacles);
	all.push_all_move(wands);
	all.push_all_move(swords);
	println!("all capacity is {}", all.capacity());
	all.shrink_to_fit();
	println!("all capacity is {}", all.capacity());
	all
}

pub fn major_arcana(window_three_forth_x:f32, window_half_y:f32) -> ~[Sprite]{
	let blank_card = Card{name:"", img_loc:""};
	let mut deck_info:[Card, ..22] = [blank_card, ..22];
	let mut deck = ~[];
	let scale_vec = Vector2f::new(0.9, 0.9);
	let mut i = 0;

	deck_info[0]	= Card{name: "Fool",img_loc: "../resources/image/00.jpg"};
	deck_info[1]	=	Card{name: "Fool",img_loc: "../resources/image/01.jpg",};
	deck_info[2]	=	Card{name: "Fool",img_loc: "../resources/image/02.jpg",};
	deck_info[3]	=	Card{name: "Fool",img_loc: "../resources/image/03.jpg",};
	deck_info[4]	=	Card{name: "Fool",img_loc: "../resources/image/04.jpg",};
	deck_info[5]	=	Card{name: "Fool",img_loc: "../resources/image/05.jpg",};
	deck_info[6]	=	Card{name: "Fool",img_loc: "../resources/image/06.jpg",};
	deck_info[7]	=	Card{name: "Fool",img_loc: "../resources/image/07.jpg",};
	deck_info[8]	=	Card{name: "Fool",img_loc: "../resources/image/08.jpg",};
	deck_info[9]	=	Card{name: "Fool",img_loc: "../resources/image/09.jpg",};
	deck_info[10]	=	Card{name: "Fool",img_loc: "../resources/image/10.jpg",};
	deck_info[11]	=	Card{name: "Fool",img_loc: "../resources/image/11.jpg",};
	deck_info[12]	=	Card{name: "Fool",img_loc: "../resources/image/12.jpg",};
	deck_info[13]	=	Card{name: "Fool",img_loc: "../resources/image/13.jpg",};
	deck_info[14]	=	Card{name: "Fool",img_loc: "../resources/image/14.jpg",};
	deck_info[15]	=	Card{name: "Fool",img_loc: "../resources/image/15.jpg",};
	deck_info[16]	=	Card{name: "Fool",img_loc: "../resources/image/16.jpg",};
	deck_info[17]	=	Card{name: "Fool",img_loc: "../resources/image/17.jpg",};
	deck_info[18]	=	Card{name: "Fool",img_loc: "../resources/image/18.jpg",};
	deck_info[19]	=	Card{name: "Fool",img_loc: "../resources/image/19.jpg",};
	deck_info[20]	=	Card{name: "Fool",img_loc: "../resources/image/20.jpg",};
	deck_info[21]	=	Card{name: "Fool",img_loc: "../resources/image/21.jpg",};

	while i < 22 {
		// Setup texture
		let current_texture = deck_info[i].texture(deck_info[i].img_loc);
		let current_ref_cell = RefCell::new(current_texture);
		let current_rc = Rc::new(current_ref_cell);
		// Setup sprite
		let mut current_sprite = deck_info[i].sprite();	// create sprite
		current_sprite.set_scale(&scale_vec);	// set scale
		current_sprite.set_texture(current_rc, false);	// set texture
		let current_bounds = current_sprite.get_local_bounds();	// Find bounds of spite
		current_sprite.set_origin2f(current_bounds.width / 2.0, current_bounds.height / 2.0);	// set origin as center of sprite
		current_sprite.set_position2f(window_three_forth_x, window_half_y);	// set position as 3/4 of window x, 1/2 of window y
		deck.push(current_sprite);
		i += 1;
	}
	deck
}

pub fn cups(window_three_forth_x:f32, window_half_y:f32) -> ~[Sprite] {
	let blank_card = Card{name:"", img_loc:""};
	let mut deck_info:[Card, ..14] = [blank_card, ..14];
	let mut deck = ~[];
	let scale_vec = Vector2f::new(0.9, 0.9);
	let mut i = 0;

	deck_info[0]	= Card{name: "Fool",img_loc: "../resources/image/c_01.jpg"};
	deck_info[1]	=	Card{name: "Fool",img_loc: "../resources/image/c_02.jpg",};
	deck_info[2]	=	Card{name: "Fool",img_loc: "../resources/image/c_03.jpg",};
	deck_info[3]	=	Card{name: "Fool",img_loc: "../resources/image/c_04.jpg",};
	deck_info[4]	=	Card{name: "Fool",img_loc: "../resources/image/c_05.jpg",};
	deck_info[5]	=	Card{name: "Fool",img_loc: "../resources/image/c_06.jpg",};
	deck_info[6]	=	Card{name: "Fool",img_loc: "../resources/image/c_07.jpg",};
	deck_info[7]	=	Card{name: "Fool",img_loc: "../resources/image/c_08.jpg",};
	deck_info[8]	=	Card{name: "Fool",img_loc: "../resources/image/c_09.jpg",};
	deck_info[9]	=	Card{name: "Fool",img_loc: "../resources/image/c_10.jpg",};
	deck_info[10]	=	Card{name: "Fool",img_loc: "../resources/image/c_page.jpg",};
	deck_info[11]	=	Card{name: "Fool",img_loc: "../resources/image/c_knight.jpg",};
	deck_info[12]	=	Card{name: "Fool",img_loc: "../resources/image/c_queen.jpg",};
	deck_info[13]	=	Card{name: "Fool",img_loc: "../resources/image/c_king.jpg",};

	while i < 14 {
		// Setup texture
		let current_texture = deck_info[i].texture(deck_info[i].img_loc);
		let current_ref_cell = RefCell::new(current_texture);
		let current_rc = Rc::new(current_ref_cell);
		// Setup sprite
		let mut current_sprite = deck_info[i].sprite();	// create sprite
		current_sprite.set_scale(&scale_vec);	// set scale
		current_sprite.set_texture(current_rc, false);	// set texture
		let current_bounds = current_sprite.get_local_bounds();	// Find bounds of spite
		current_sprite.set_origin2f(current_bounds.width / 2.0, current_bounds.height / 2.0);	// set origin as center of sprite
		current_sprite.set_position2f(window_three_forth_x, window_half_y);	// set position as 3/4 of window x, 1/2 of window y
		deck.push(current_sprite);
		i += 1;
	}
	deck
}
pub fn pentacles(window_three_forth_x:f32, window_half_y:f32) -> ~[Sprite] {
	let blank_card = Card{name:"", img_loc:""};
	let mut deck_info:[Card, ..14] = [blank_card, ..14];
	let mut deck = ~[];
	let scale_vec = Vector2f::new(0.9, 0.9);
	let mut i = 0;

	deck_info[0]	= Card{name: "Fool",img_loc: "../resources/image/p_01.jpg"};
	deck_info[1]	=	Card{name: "Fool",img_loc: "../resources/image/p_02.jpg",};
	deck_info[2]	=	Card{name: "Fool",img_loc: "../resources/image/p_03.jpg",};
	deck_info[3]	=	Card{name: "Fool",img_loc: "../resources/image/p_04.jpg",};
	deck_info[4]	=	Card{name: "Fool",img_loc: "../resources/image/p_05.jpg",};
	deck_info[5]	=	Card{name: "Fool",img_loc: "../resources/image/p_06.jpg",};
	deck_info[6]	=	Card{name: "Fool",img_loc: "../resources/image/p_07.jpg",};
	deck_info[7]	=	Card{name: "Fool",img_loc: "../resources/image/p_08.jpg",};
	deck_info[8]	=	Card{name: "Fool",img_loc: "../resources/image/p_09.jpg",};
	deck_info[9]	=	Card{name: "Fool",img_loc: "../resources/image/p_10.jpg",};
	deck_info[10]	=	Card{name: "Fool",img_loc: "../resources/image/p_page.jpg",};
	deck_info[11]	=	Card{name: "Fool",img_loc: "../resources/image/p_knight.jpg",};
	deck_info[12]	=	Card{name: "Fool",img_loc: "../resources/image/p_queen.jpg",};
	deck_info[13]	=	Card{name: "Fool",img_loc: "../resources/image/p_king.jpg",};

	while i < 14 {
		// Setup texture
		let current_texture = deck_info[i].texture(deck_info[i].img_loc);
		let current_ref_cell = RefCell::new(current_texture);
		let current_rc = Rc::new(current_ref_cell);
		// Setup sprite
		let mut current_sprite = deck_info[i].sprite();	// create sprite
		current_sprite.set_scale(&scale_vec);	// set scale
		current_sprite.set_texture(current_rc, false);	// set texture
		let current_bounds = current_sprite.get_local_bounds();	// Find bounds of spite
		current_sprite.set_origin2f(current_bounds.width / 2.0, current_bounds.height / 2.0);	// set origin as center of sprite
		current_sprite.set_position2f(window_three_forth_x, window_half_y);	// set position as 3/4 of window x, 1/2 of window y
		deck.push(current_sprite);
		i += 1;
	}
	deck
}
pub fn swords(window_three_forth_x:f32, window_half_y:f32) -> ~[Sprite] {
	let blank_card = Card{name:"", img_loc:""};
	let mut deck_info:[Card, ..14] = [blank_card, ..14];
	let mut deck = ~[];
	let scale_vec = Vector2f::new(0.9, 0.9);
	let mut i = 0;

	deck_info[0]	= Card{name: "Fool",img_loc: "../resources/image/s_01.jpg"};
	deck_info[1]	=	Card{name: "Fool",img_loc: "../resources/image/s_02.jpg",};
	deck_info[2]	=	Card{name: "Fool",img_loc: "../resources/image/s_03.jpg",};
	deck_info[3]	=	Card{name: "Fool",img_loc: "../resources/image/s_04.jpg",};
	deck_info[4]	=	Card{name: "Fool",img_loc: "../resources/image/s_05.jpg",};
	deck_info[5]	=	Card{name: "Fool",img_loc: "../resources/image/s_06.jpg",};
	deck_info[6]	=	Card{name: "Fool",img_loc: "../resources/image/s_07.jpg",};
	deck_info[7]	=	Card{name: "Fool",img_loc: "../resources/image/s_08.jpg",};
	deck_info[8]	=	Card{name: "Fool",img_loc: "../resources/image/s_09.jpg",};
	deck_info[9]	=	Card{name: "Fool",img_loc: "../resources/image/s_10.jpg",};
	deck_info[10]	=	Card{name: "Fool",img_loc: "../resources/image/s_page.jpg",};
	deck_info[11]	=	Card{name: "Fool",img_loc: "../resources/image/s_knight.jpg",};
	deck_info[12]	=	Card{name: "Fool",img_loc: "../resources/image/s_queen.jpg",};
	deck_info[13]	=	Card{name: "Fool",img_loc: "../resources/image/s_king.jpg",};

	while i < 14 {
		// Setup texture
		let current_texture = deck_info[i].texture(deck_info[i].img_loc);
		let current_ref_cell = RefCell::new(current_texture);
		let current_rc = Rc::new(current_ref_cell);
		// Setup sprite
		let mut current_sprite = deck_info[i].sprite();	// create sprite
		current_sprite.set_scale(&scale_vec);	// set scale
		current_sprite.set_texture(current_rc, false);	// set texture
		let current_bounds = current_sprite.get_local_bounds();	// Find bounds of spite
		current_sprite.set_origin2f(current_bounds.width / 2.0, current_bounds.height / 2.0);	// set origin as center of sprite
		current_sprite.set_position2f(window_three_forth_x, window_half_y);	// set position as 3/4 of window x, 1/2 of window y
		deck.push(current_sprite);
		i += 1;
	}
	deck
}
pub fn wands(window_three_forth_x:f32, window_half_y:f32) -> ~[Sprite] {
	let blank_card = Card{name:"", img_loc:""};
	let mut deck_info:[Card, ..14] = [blank_card, ..14];
	let mut deck = ~[];
	let scale_vec = Vector2f::new(0.9, 0.9);
	let mut i = 0;

	deck_info[0]	= Card{name: "Fool",img_loc: "../resources/image/w_01.jpg"};
	deck_info[1]	=	Card{name: "Fool",img_loc: "../resources/image/w_02.jpg",};
	deck_info[2]	=	Card{name: "Fool",img_loc: "../resources/image/w_03.jpg",};
	deck_info[3]	=	Card{name: "Fool",img_loc: "../resources/image/w_04.jpg",};
	deck_info[4]	=	Card{name: "Fool",img_loc: "../resources/image/w_05.jpg",};
	deck_info[5]	=	Card{name: "Fool",img_loc: "../resources/image/w_06.jpg",};
	deck_info[6]	=	Card{name: "Fool",img_loc: "../resources/image/w_07.jpg",};
	deck_info[7]	=	Card{name: "Fool",img_loc: "../resources/image/w_08.jpg",};
	deck_info[8]	=	Card{name: "Fool",img_loc: "../resources/image/w_09.jpg",};
	deck_info[9]	=	Card{name: "Fool",img_loc: "../resources/image/w_10.jpg",};
	deck_info[10]	=	Card{name: "Fool",img_loc: "../resources/image/w_page.jpg",};
	deck_info[11]	=	Card{name: "Fool",img_loc: "../resources/image/w_knight.jpg",};
	deck_info[12]	=	Card{name: "Fool",img_loc: "../resources/image/w_queen.jpg",};
	deck_info[13]	=	Card{name: "Fool",img_loc: "../resources/image/w_king.jpg",};

	while i < 14 {
		// Setup texture
		let current_texture = deck_info[i].texture(deck_info[i].img_loc);
		let current_ref_cell = RefCell::new(current_texture);
		let current_rc = Rc::new(current_ref_cell);
		// Setup sprite
		let mut current_sprite = deck_info[i].sprite();	// create sprite
		current_sprite.set_scale(&scale_vec);	// set scale
		current_sprite.set_texture(current_rc, false);	// set texture
		let current_bounds = current_sprite.get_local_bounds();	// Find bounds of spite
		current_sprite.set_origin2f(current_bounds.width / 2.0, current_bounds.height / 2.0);	// set origin as center of sprite
		current_sprite.set_position2f(window_three_forth_x, window_half_y);	// set position as 3/4 of window x, 1/2 of window y
		deck.push(current_sprite);
		i += 1;
	}
	deck
}