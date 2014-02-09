extern mod rsfml;
use rsfml::window::{event, keyboard, mouse};
use rsfml::graphics::{RenderWindow};
use rsfml::graphics::rc::{Sprite};

pub fn exit(window: &mut RenderWindow) {
	loop {
		match window.poll_event() {
			event::Closed	=> window.close(),
			event::NoEvent	=> break,
			_				=> {}
		}
	}
}

pub fn menu() -> uint {
	let mut screen:uint = 1;
	if keyboard::is_key_pressed(keyboard::Space){screen = 2}
	if keyboard::is_key_pressed(keyboard::U){screen = 3}
	screen
}

pub fn card_shift(window:&mut RenderWindow) -> (int, uint) {
	let mut move = 0; let mut screen:uint = 2;

	if keyboard::is_key_pressed(keyboard::D){move = 1}
	if keyboard::is_key_pressed(keyboard::A){move = -1}
	if keyboard::is_key_pressed(keyboard::Escape){screen = 5}
	(move, screen)
}

pub fn reading() -> uint {
	let mut screen:uint = 3;
	if keyboard::is_key_pressed(keyboard::Escape){screen = 5}
	screen
}
