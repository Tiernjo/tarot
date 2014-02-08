extern mod rsfml;
use rsfml::window::{event, keyboard};
use rsfml::graphics::{RenderWindow};
use rsfml::graphics::rc::{Sprite};

pub fn exit(window: &mut RenderWindow) {
	if keyboard::is_key_pressed(keyboard::Escape) {window.close()}
	loop {
		match window.poll_event() {
			event::Closed	=> window.close(),
			event::NoEvent	=> break,
			_				=> {}
		}
	}
}

pub fn menu() -> (bool, bool){
	if keyboard::is_key_pressed(keyboard::Space){return (false, true)}
	(true, false)
}

pub fn card_right() -> bool {
	let mut is_next_card:bool = false;
	if keyboard::is_key_pressed(keyboard::D) {is_next_card = true}
	is_next_card
}
pub fn card_left() -> bool {
	let mut is_last_card:bool = false;
	if keyboard::is_key_pressed(keyboard::A) {is_last_card = true}
	is_last_card
}
