extern mod rsfml;
use rsfml::window::{event, keyboard};
use rsfml::graphics::{RenderWindow};

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

pub fn menu(window: &mut RenderWindow) -> (bool, bool){
	if keyboard::is_key_pressed(keyboard::Space){return (false, true)}
	(true, false)
}
