extern mod rsfml;
use rsfml::graphics::{Sprite, Texture};

struct Card{
	name:&str,
	location:&str
}
impl Card {
	fn texture(&self) -> Texture{
		let texture = match Texture::new_from_file() {
			Some(texture)	=>	texture,
			None()			=>	fail!(~"Error, card block")
		};
		texture
	}
	fn sprite(&self) -> Sprite {
		let sprite = match Sprite::new_from_texture() {
			Some(sprite)	=>	sprite,
			None()			=>	fail!(~"Error, card sprite"),
		};
		sprite
	}
}

pub fn one() {
}