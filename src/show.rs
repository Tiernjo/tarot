extern mod rsfml;
use rsfml::graphics::{RenderWindow, Texture};
use rsfml::graphics::rc::Sprite;
use rsfml::system::{Vector2f};
use std::cell::RefCell;
use std::rc::Rc;

struct Card<'s>{
	//name:&'s str,
	location:&'s str
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

pub fn one(window_three_forth_x:f32, window_half_y:f32, card_number:int) -> Sprite {
	let (mut current_location, mut current_name) = ("../resources/image/00.jpg", "Fool");
	let scale_vec = Vector2f::new(0.9, 0.9);
	
	match card_number {
		0	=>	{current_name = "Fool";current_location = "../resources/image/00.jpg";}
		1	=>	{current_name = "Fool";current_location = "../resources/image/01.jpg";}
		2	=>	{current_name = "Fool";current_location = "../resources/image/02.jpg";}
		3	=>	{current_name = "Fool";current_location = "../resources/image/03.jpg";}
		4	=>	{current_name = "Fool";current_location = "../resources/image/04.jpg";}
		5	=>	{current_name = "Fool";current_location = "../resources/image/05.jpg";}
		6	=>	{current_name = "Fool";current_location = "../resources/image/06.jpg";}
		7	=>	{current_name = "Fool";current_location = "../resources/image/07.jpg";}
		8	=>	{current_name = "Fool";current_location = "../resources/image/08.jpg";}
		9	=>	{current_name = "Fool";current_location = "../resources/image/09.jpg";}
		10	=>	{current_name = "Fool";current_location = "../resources/image/10.jpg";}
		11	=>	{current_name = "Fool";current_location = "../resources/image/11.jpg";}
		12	=>	{current_name = "Fool";current_location = "../resources/image/12.jpg";}
		13	=>	{current_name = "Fool";current_location = "../resources/image/13.jpg";}
		14	=>	{current_name = "Fool";current_location = "../resources/image/14.jpg";}
		15	=>	{current_name = "Fool";current_location = "../resources/image/15.jpg";}
		16	=>	{current_name = "Fool";current_location = "../resources/image/16.jpg";}
		17	=>	{current_name = "Fool";current_location = "../resources/image/17.jpg";}
		18	=>	{current_name = "Fool";current_location = "../resources/image/18.jpg";}
		19	=>	{current_name = "Fool";current_location = "../resources/image/19.jpg";}
		20	=>	{current_name = "Fool";current_location = "../resources/image/20.jpg";}
		21	=>	{current_name = "Fool";current_location = "../resources/image/21.jpg";}
		22	=>	{current_name = "Fool";current_location = "../resources/image/c_01.jpg";}
		23	=>	{current_name = "Fool";current_location = "../resources/image/c_02.jpg";}
		24	=>	{current_name = "Fool";current_location = "../resources/image/c_03.jpg";}
		25	=>	{current_name = "Fool";current_location = "../resources/image/c_04.jpg";}
		26	=>	{current_name = "Fool";current_location = "../resources/image/c_05.jpg";}
		27	=>	{current_name = "Fool";current_location = "../resources/image/c_06.jpg";}
		28	=>	{current_name = "Fool";current_location = "../resources/image/c_07.jpg";}
		29	=>	{current_name = "Fool";current_location = "../resources/image/c_08.jpg";}
		30	=>	{current_name = "Fool";current_location = "../resources/image/c_09.jpg";}
		31	=>	{current_name = "Fool";current_location = "../resources/image/c_10.jpg";}
		32	=>	{current_name = "Fool";current_location = "../resources/image/c_page.jpg";}
		33	=>	{current_name = "Fool";current_location = "../resources/image/c_knight.jpg";}
		34	=>	{current_name = "Fool";current_location = "../resources/image/c_queen.jpg";}
		35	=>	{current_name = "Fool";current_location = "../resources/image/c_king.jpg";}
		36	=>	{current_name = "Fool";current_location = "../resources/image/p_01.jpg";}
		37	=>	{current_name = "Fool";current_location = "../resources/image/p_02.jpg";}
		38	=>	{current_name = "Fool";current_location = "../resources/image/p_03.jpg";}
		39	=>	{current_name = "Fool";current_location = "../resources/image/p_04.jpg";}
		40	=>	{current_name = "Fool";current_location = "../resources/image/p_05.jpg";}
		41	=>	{current_name = "Fool";current_location = "../resources/image/p_06.jpg";}
		42	=>	{current_name = "Fool";current_location = "../resources/image/p_07.jpg";}
		43	=>	{current_name = "Fool";current_location = "../resources/image/p_08.jpg";}
		44	=>	{current_name = "Fool";current_location = "../resources/image/p_09.jpg";}
		45	=>	{current_name = "Fool";current_location = "../resources/image/p_10.jpg";}
		46	=>	{current_name = "Fool";current_location = "../resources/image/p_page.jpg";}
		47	=>	{current_name = "Fool";current_location = "../resources/image/p_knight.jpg";}
		48	=>	{current_name = "Fool";current_location = "../resources/image/p_queen.jpg";}
		49	=>	{current_name = "Fool";current_location = "../resources/image/p_king.jpg";}
		50	=>	{current_name = "Fool";current_location = "../resources/image/s_01.jpg";}
		51	=>	{current_name = "Fool";current_location = "../resources/image/s_02.jpg";}
		52	=>	{current_name = "Fool";current_location = "../resources/image/s_03.jpg";}
		53	=>	{current_name = "Fool";current_location = "../resources/image/s_04.jpg";}
		54	=>	{current_name = "Fool";current_location = "../resources/image/s_05.jpg";}
		55	=>	{current_name = "Fool";current_location = "../resources/image/s_06.jpg";}
		56	=>	{current_name = "Fool";current_location = "../resources/image/s_07.jpg";}
		57	=>	{current_name = "Fool";current_location = "../resources/image/s_08.jpg";}
		58	=>	{current_name = "Fool";current_location = "../resources/image/s_09.jpg";}
		59	=>	{current_name = "Fool";current_location = "../resources/image/s_10.jpg";}
		60	=>	{current_name = "Fool";current_location = "../resources/image/s_page.jpg";}
		61	=>	{current_name = "Fool";current_location = "../resources/image/s_knight.jpg";}
		62	=>	{current_name = "Fool";current_location = "../resources/image/s_queen.jpg";}
		63	=>	{current_name = "Fool";current_location = "../resources/image/s_king.jpg";}
		64	=>	{current_name = "Fool";current_location = "../resources/image/w_01.jpg";}
		65	=>	{current_name = "Fool";current_location = "../resources/image/w_02.jpg";}
		66	=>	{current_name = "Fool";current_location = "../resources/image/w_03.jpg";}
		67	=>	{current_name = "Fool";current_location = "../resources/image/w_04.jpg";}
		68	=>	{current_name = "Fool";current_location = "../resources/image/w_05.jpg";}
		69	=>	{current_name = "Fool";current_location = "../resources/image/w_06.jpg";}
		70	=>	{current_name = "Fool";current_location = "../resources/image/w_07.jpg";}
		71	=>	{current_name = "Fool";current_location = "../resources/image/w_08.jpg";}
		72	=>	{current_name = "Fool";current_location = "../resources/image/w_09.jpg";}
		73	=>	{current_name = "Fool";current_location = "../resources/image/w_10.jpg";}
		74	=>	{current_name = "Fool";current_location = "../resources/image/w_page.jpg";}
		75	=>	{current_name = "Fool";current_location = "../resources/image/w_knight.jpg";}
		76	=>	{current_name = "Fool";current_location = "../resources/image/w_queen.jpg";}
		77	=>	{current_name = "Fool";current_location = "../resources/image/w_king.jpg";}
		_	=>	fail!(~"Error, finding card location")
	}

	let current_card = Card{location:current_location};
	// Setup texture
	let current_texture = current_card.texture(current_location);
	let current_ref_cell = RefCell::new(current_texture);
	let current_rc = Rc::new(current_ref_cell);
	// Setup sprite
	let mut current_sprite = current_card.sprite();	// create sprite
	current_sprite.set_scale(&scale_vec);	// set scale
	current_sprite.set_texture(current_rc, false);	// set texture
	let current_bounds = current_sprite.get_local_bounds();	// Find bounds of spite
	current_sprite.set_origin2f(current_bounds.width / 2.0, current_bounds.height / 2.0);	// set origin as center of sprite
	current_sprite.set_position2f(window_three_forth_x, window_half_y);	// set position as 3/4 of window x, 1/2 of window y
	current_sprite
}