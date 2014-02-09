extern mod rsfml;
use rsfml::window::{event, keyboard, mouse};
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

pub fn menu() -> uint {
	let mut screen:uint = 1;
	if keyboard::is_key_pressed(keyboard::Space){screen = 2}
	screen
}

pub fn card_shift(window:&mut RenderWindow) -> int {
	let mut move = 0;

	loop {
		match window.poll_event() {
			event::KeyPressed{code, ..}	=> match code {
				keyboard::D		=>	{move = 1}
				keyboard::Right	=>	{move = 1}
				keyboard::A		=>	{move = -1}
				keyboard::Left	=>	{move = -1}
				_			=>	{}
			},
			event::NoEvent				=>	break,
			_							=>	{}
		}
	}
	if keyboard::is_key_pressed(keyboard::D){move = 1}
	if keyboard::is_key_pressed(keyboard::A){move = -1}
	move
}
