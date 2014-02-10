extern mod rsfml;
use rsfml::graphics::{Texture};
use rsfml::graphics::rc::{Sprite, Text};
use rsfml::system::{Vector2f};
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use std::io::{BufferedReader, File};

mod menu;

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

pub fn new(window_fourth_x:f32, window_three_forth_x:f32, window_fourth_y:f32, window_half_y:f32) -> (~[Sprite], ~[Text]) {
	let (major_arcana, major_arcana_desc) = major_arcana(window_fourth_x, window_three_forth_x, window_fourth_y, window_half_y);
	let (cups, cups_desc) = cups(window_fourth_x, window_three_forth_x, window_fourth_y, window_half_y);
	let (pentacles, pentacles_desc) = pentacles(window_fourth_x, window_three_forth_x, window_fourth_y, window_half_y);
	let (wands, wands_desc) = wands(window_fourth_x, window_three_forth_x, window_fourth_y, window_half_y);
	let (swords, swords_desc) = swords(window_fourth_x, window_three_forth_x, window_fourth_y, window_half_y);

	// Combine all vectors
	let mut all = ~[]; let mut all_desc = ~[];
	all.push_all_move(major_arcana); all_desc.push_all_move(major_arcana_desc);
	all.push_all_move(cups);all_desc.push_all_move(cups_desc);
	all.push_all_move(pentacles);all_desc.push_all_move(pentacles_desc);
	all.push_all_move(wands);all_desc.push_all_move(wands_desc);
	all.push_all_move(swords);all_desc.push_all_move(swords_desc);

	// Remove empty indexes
	all.shrink_to_fit();
	all_desc.shrink_to_fit();
	(all, all_desc)
}

fn major_arcana(window_fourth_x:f32, window_three_forth_x:f32, window_fourth_y:f32, window_half_y:f32) -> (~[Sprite], ~[Text]){
	let blank_card = Card{name:"", img_loc:"",str_loc:""};
	let mut deck_info:[Card, ..22] = [blank_card, ..22];
	let mut deck = ~[];
	let mut description = ~[];
	let scale_vec = Vector2f::new(0.9, 0.9);
	let mut i = 0;

	deck_info[0]	=	Card{name: "Fool",img_loc: "../resources/image/00.jpg",str_loc: "../resources/text/00.txt"};
	deck_info[1]	=	Card{name: "Fool",img_loc: "../resources/image/01.jpg",str_loc: "../resources/text/01.txt"};
	deck_info[2]	=	Card{name: "Fool",img_loc: "../resources/image/02.jpg",str_loc: "../resources/text/02.txt"};
	deck_info[3]	=	Card{name: "Fool",img_loc: "../resources/image/03.jpg",str_loc: "../resources/text/03.txt"};
	deck_info[4]	=	Card{name: "Fool",img_loc: "../resources/image/04.jpg",str_loc: "../resources/text/04.txt"};
	deck_info[5]	=	Card{name: "Fool",img_loc: "../resources/image/05.jpg",str_loc: "../resources/text/05.txt"};
	deck_info[6]	=	Card{name: "Fool",img_loc: "../resources/image/06.jpg",str_loc: "../resources/text/06.txt"};
	deck_info[7]	=	Card{name: "Fool",img_loc: "../resources/image/07.jpg",str_loc: "../resources/text/07.txt"};
	deck_info[8]	=	Card{name: "Fool",img_loc: "../resources/image/08.jpg",str_loc: "../resources/text/08.txt"};
	deck_info[9]	=	Card{name: "Fool",img_loc: "../resources/image/09.jpg",str_loc: "../resources/text/09.txt"};
	deck_info[10]	=	Card{name: "Fool",img_loc: "../resources/image/10.jpg",str_loc: "../resources/text/10.txt"};
	deck_info[11]	=	Card{name: "Fool",img_loc: "../resources/image/11.jpg",str_loc: "../resources/text/11.txt"};
	deck_info[12]	=	Card{name: "Fool",img_loc: "../resources/image/12.jpg",str_loc: "../resources/text/12.txt"};
	deck_info[13]	=	Card{name: "Fool",img_loc: "../resources/image/13.jpg",str_loc: "../resources/text/13.txt"};
	deck_info[14]	=	Card{name: "Fool",img_loc: "../resources/image/14.jpg",str_loc: "../resources/text/14.txt"};
	deck_info[15]	=	Card{name: "Fool",img_loc: "../resources/image/15.jpg",str_loc: "../resources/text/15.txt"};
	deck_info[16]	=	Card{name: "Fool",img_loc: "../resources/image/16.jpg",str_loc: "../resources/text/16.txt"};
	deck_info[17]	=	Card{name: "Fool",img_loc: "../resources/image/17.jpg",str_loc: "../resources/text/17.txt"};
	deck_info[18]	=	Card{name: "Fool",img_loc: "../resources/image/18.jpg",str_loc: "../resources/text/18.txt"};
	deck_info[19]	=	Card{name: "Fool",img_loc: "../resources/image/19.jpg",str_loc: "../resources/text/19.txt"};
	deck_info[20]	=	Card{name: "Fool",img_loc: "../resources/image/20.jpg",str_loc: "../resources/text/20.txt"};
	deck_info[21]	=	Card{name: "Fool",img_loc: "../resources/image/21.jpg",str_loc: "../resources/text/21.txt"};

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
		// Setup Text
		let path:Path = Path::new(deck_info[i].str_loc);	// Create a path to file
		let mut file = BufferedReader::new(File::open(&path));	// Create File Reader
		let lines:~[~str] = file.lines().collect();	// Write each line into a 
		let current_text = ::menu::new("../resources/font/Jura-DemiBold.ttf", lines[0], 30, window_fourth_x, window_fourth_y, 0.0);
		description.push(current_text);
		i += 1;
	}
	(deck, description)
}

fn cups(window_fourth_x:f32, window_three_forth_x:f32, window_fourth_y:f32, window_half_y:f32) -> (~[Sprite], ~[Text]){
	let blank_card = Card{name:"", img_loc:"",str_loc:""};
	let mut deck_info:[Card, ..14] = [blank_card, ..14];
	let mut deck = ~[];
	let mut description = ~[];
	let scale_vec = Vector2f::new(0.9, 0.9);
	let mut i = 0;

	deck_info[0]	=	Card{name: "Fool",img_loc: "../resources/image/c_01.jpg",str_loc: "../resources/text/c_01.txt"};
	deck_info[1]	=	Card{name: "Fool",img_loc: "../resources/image/c_02.jpg",str_loc: "../resources/text/c_02.txt"};
	deck_info[2]	=	Card{name: "Fool",img_loc: "../resources/image/c_03.jpg",str_loc: "../resources/text/c_03.txt"};
	deck_info[3]	=	Card{name: "Fool",img_loc: "../resources/image/c_04.jpg",str_loc: "../resources/text/c_04.txt"};
	deck_info[4]	=	Card{name: "Fool",img_loc: "../resources/image/c_05.jpg",str_loc: "../resources/text/c_05.txt"};
	deck_info[5]	=	Card{name: "Fool",img_loc: "../resources/image/c_06.jpg",str_loc: "../resources/text/c_06.txt"};
	deck_info[6]	=	Card{name: "Fool",img_loc: "../resources/image/c_07.jpg",str_loc: "../resources/text/c_07.txt"};
	deck_info[7]	=	Card{name: "Fool",img_loc: "../resources/image/c_08.jpg",str_loc: "../resources/text/c_08.txt"};
	deck_info[8]	=	Card{name: "Fool",img_loc: "../resources/image/c_09.jpg",str_loc: "../resources/text/c_09.txt"};
	deck_info[9]	=	Card{name: "Fool",img_loc: "../resources/image/c_10.jpg",str_loc: "../resources/text/c_10.txt"};
	deck_info[10]	=	Card{name: "Fool",img_loc: "../resources/image/c_page.jpg",str_loc: "../resources/text/c_page.txt"};
	deck_info[11]	=	Card{name: "Fool",img_loc: "../resources/image/c_knight.jpg",str_loc: "../resources/text/c_knight.txt"};
	deck_info[12]	=	Card{name: "Fool",img_loc: "../resources/image/c_queen.jpg",str_loc: "../resources/text/c_queen.txt"};
	deck_info[13]	=	Card{name: "Fool",img_loc: "../resources/image/c_king.jpg",str_loc: "../resources/text/c_king.txt"};

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
		// Setup Text
		let path:Path = Path::new(deck_info[i].str_loc);
		let mut file = BufferedReader::new(File::open(&path));
		let lines:~[~str] = file.lines().collect();
		let current_text = ::menu::new("../resources/font/Jura-DemiBold.ttf", lines[0], 30, window_fourth_x, window_fourth_y, 0.0);
		description.push(current_text);
		i += 1;
	}
	(deck, description)
}
fn pentacles(window_fourth_x:f32, window_three_forth_x:f32, window_fourth_y:f32, window_half_y:f32) -> (~[Sprite], ~[Text]) {
	let blank_card = Card{name:"", img_loc:"",str_loc:""};
	let mut deck_info:[Card, ..14] = [blank_card, ..14];
	let mut deck = ~[];
	let mut description = ~[];
	let scale_vec = Vector2f::new(0.9, 0.9);
	let mut i = 0;

	deck_info[0]	=	Card{name: "Fool",img_loc: "../resources/image/p_01.jpg",str_loc: "../resources/text/p_01.txt"};
	deck_info[1]	=	Card{name: "Fool",img_loc: "../resources/image/p_02.jpg",str_loc: "../resources/text/p_02.txt"};
	deck_info[2]	=	Card{name: "Fool",img_loc: "../resources/image/p_03.jpg",str_loc: "../resources/text/p_03.txt"};
	deck_info[3]	=	Card{name: "Fool",img_loc: "../resources/image/p_04.jpg",str_loc: "../resources/text/p_04.txt"};
	deck_info[4]	=	Card{name: "Fool",img_loc: "../resources/image/p_05.jpg",str_loc: "../resources/text/p_05.txt"};
	deck_info[5]	=	Card{name: "Fool",img_loc: "../resources/image/p_06.jpg",str_loc: "../resources/text/p_06.txt"};
	deck_info[6]	=	Card{name: "Fool",img_loc: "../resources/image/p_07.jpg",str_loc: "../resources/text/p_07.txt"};
	deck_info[7]	=	Card{name: "Fool",img_loc: "../resources/image/p_08.jpg",str_loc: "../resources/text/p_08.txt"};
	deck_info[8]	=	Card{name: "Fool",img_loc: "../resources/image/p_09.jpg",str_loc: "../resources/text/p_09.txt"};
	deck_info[9]	=	Card{name: "Fool",img_loc: "../resources/image/p_10.jpg",str_loc: "../resources/text/p_10.txt"};
	deck_info[10]	=	Card{name: "Fool",img_loc: "../resources/image/p_page.jpg",str_loc: "../resources/text/p_page.txt"};
	deck_info[11]	=	Card{name: "Fool",img_loc: "../resources/image/p_knight.jpg",str_loc: "../resources/text/p_knight.txt"};
	deck_info[12]	=	Card{name: "Fool",img_loc: "../resources/image/p_queen.jpg",str_loc: "../resources/text/p_queen.txt"};
	deck_info[13]	=	Card{name: "Fool",img_loc: "../resources/image/p_king.jpg",str_loc: "../resources/text/p_king.txt"};

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
		// Setup Text
		let path:Path = Path::new(deck_info[i].str_loc);
		let mut file = BufferedReader::new(File::open(&path));
		let lines:~[~str] = file.lines().collect();
		let current_text = ::menu::new("../resources/font/Jura-DemiBold.ttf", lines[0], 30, window_fourth_x, window_fourth_y, 0.0);
		description.push(current_text);
		i += 1;
	}
	(deck, description)
}
fn swords(window_fourth_x:f32, window_three_forth_x:f32, window_fourth_y:f32, window_half_y:f32) -> (~[Sprite], ~[Text]) {
	let blank_card = Card{name:"", img_loc:"",str_loc:""};
	let mut deck_info:[Card, ..14] = [blank_card, ..14];
	let mut deck = ~[];
	let mut description = ~[];
	let scale_vec = Vector2f::new(0.9, 0.9);
	let mut i = 0;

	deck_info[0]	=	Card{name: "Fool",img_loc: "../resources/image/s_01.jpg",str_loc: "../resources/text/s_01.txt"};
	deck_info[1]	=	Card{name: "Fool",img_loc: "../resources/image/s_02.jpg",str_loc: "../resources/text/s_02.txt"};
	deck_info[2]	=	Card{name: "Fool",img_loc: "../resources/image/s_03.jpg",str_loc: "../resources/text/s_03.txt"};
	deck_info[3]	=	Card{name: "Fool",img_loc: "../resources/image/s_04.jpg",str_loc: "../resources/text/s_04.txt"};
	deck_info[4]	=	Card{name: "Fool",img_loc: "../resources/image/s_05.jpg",str_loc: "../resources/text/s_05.txt"};
	deck_info[5]	=	Card{name: "Fool",img_loc: "../resources/image/s_06.jpg",str_loc: "../resources/text/s_06.txt"};
	deck_info[6]	=	Card{name: "Fool",img_loc: "../resources/image/s_07.jpg",str_loc: "../resources/text/s_07.txt"};
	deck_info[7]	=	Card{name: "Fool",img_loc: "../resources/image/s_08.jpg",str_loc: "../resources/text/s_08.txt"};
	deck_info[8]	=	Card{name: "Fool",img_loc: "../resources/image/s_09.jpg",str_loc: "../resources/text/s_09.txt"};
	deck_info[9]	=	Card{name: "Fool",img_loc: "../resources/image/s_10.jpg",str_loc: "../resources/text/s_10.txt"};
	deck_info[10]	=	Card{name: "Fool",img_loc: "../resources/image/s_page.jpg",str_loc: "../resources/text/s_page.txt"};
	deck_info[11]	=	Card{name: "Fool",img_loc: "../resources/image/s_knight.jpg",str_loc: "../resources/text/s_knight.txt"};
	deck_info[12]	=	Card{name: "Fool",img_loc: "../resources/image/s_queen.jpg",str_loc: "../resources/text/s_queen.txt"};
	deck_info[13]	=	Card{name: "Fool",img_loc: "../resources/image/s_king.jpg",str_loc: "../resources/text/s_king.txt"};

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
		// Setup Text
		let path:Path = Path::new(deck_info[i].str_loc);
		let mut file = BufferedReader::new(File::open(&path));
		let lines:~[~str] = file.lines().collect();
		let current_text = ::menu::new("../resources/font/Jura-DemiBold.ttf", lines[0], 30, window_fourth_x, window_fourth_y, 0.0);
		description.push(current_text);
		i += 1;
	}
	(deck, description)
}
fn wands(window_fourth_x:f32, window_three_forth_x:f32, window_fourth_y:f32, window_half_y:f32) -> (~[Sprite], ~[Text]) {
	let blank_card = Card{name:"", img_loc:"",str_loc:""};
	let mut deck_info:[Card, ..14] = [blank_card, ..14];
	let mut deck = ~[];
	let mut description = ~[];
	let scale_vec = Vector2f::new(0.9, 0.9);
	let mut i = 0;

	deck_info[0]	=	Card{name: "Fool",img_loc: "../resources/image/w_01.jpg",str_loc: "../resources/text/w_01.txt"};
	deck_info[1]	=	Card{name: "Fool",img_loc: "../resources/image/w_02.jpg",str_loc: "../resources/text/w_02.txt"};
	deck_info[2]	=	Card{name: "Fool",img_loc: "../resources/image/w_03.jpg",str_loc: "../resources/text/w_03.txt"};
	deck_info[3]	=	Card{name: "Fool",img_loc: "../resources/image/w_04.jpg",str_loc: "../resources/text/w_04.txt"};
	deck_info[4]	=	Card{name: "Fool",img_loc: "../resources/image/w_05.jpg",str_loc: "../resources/text/w_05.txt"};
	deck_info[5]	=	Card{name: "Fool",img_loc: "../resources/image/w_06.jpg",str_loc: "../resources/text/w_06.txt"};
	deck_info[6]	=	Card{name: "Fool",img_loc: "../resources/image/w_07.jpg",str_loc: "../resources/text/w_07.txt"};
	deck_info[7]	=	Card{name: "Fool",img_loc: "../resources/image/w_08.jpg",str_loc: "../resources/text/w_08.txt"};
	deck_info[8]	=	Card{name: "Fool",img_loc: "../resources/image/w_09.jpg",str_loc: "../resources/text/w_09.txt"};
	deck_info[9]	=	Card{name: "Fool",img_loc: "../resources/image/w_10.jpg",str_loc: "../resources/text/w_10.txt"};
	deck_info[10]	=	Card{name: "Fool",img_loc: "../resources/image/w_page.jpg",str_loc: "../resources/text/w_page.txt"};
	deck_info[11]	=	Card{name: "Fool",img_loc: "../resources/image/w_knight.jpg",str_loc: "../resources/text/w_knight.txt"};
	deck_info[12]	=	Card{name: "Fool",img_loc: "../resources/image/w_queen.jpg",str_loc: "../resources/text/w_queen.txt"};
	deck_info[13]	=	Card{name: "Fool",img_loc: "../resources/image/w_king.jpg",str_loc: "../resources/text/w_king.txt"};

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
		// Setup Text
		let path:Path = Path::new(deck_info[i].str_loc);
		let mut file = BufferedReader::new(File::open(&path));
		let lines:~[~str] = file.lines().collect();
		let current_text = ::menu::new("../resources/font/Jura-DemiBold.ttf", lines[0], 30, window_fourth_x, window_fourth_y, 0.0);
		description.push(current_text);
		i += 1;
	}
	(deck, description)
}