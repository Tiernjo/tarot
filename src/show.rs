extern mod rsfml;
use rsfml::graphics::{Texture};
use rsfml::graphics::rc::Sprite;
use rsfml::system::{Vector2f};
use std::cell::RefCell;
use std::rc::Rc;

struct Card<'s>{
	name:&'s str,
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

pub fn one(card_number:int) -> Sprite {
	let (mut current_location, mut current_name) = ("../resources/image/00.jpg", "Fool");
	let scale_vec = Vector2f::new(0.80, 0.80);
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
		_	=>	fail!(~"Error, finding card location")
	}

	let current_card = Card{name:current_name, location:current_location};
	// Setup texture
	let current_texture = current_card.texture(current_location);
	let current_ref_cell = RefCell::new(current_texture);
	let current_rc = Rc::new(current_ref_cell);
	// Setup sprite
	let mut current_sprite = current_card.sprite();
	current_sprite.set_texture(current_rc, false);
	current_sprite.set_scale(&scale_vec);
	current_sprite
}