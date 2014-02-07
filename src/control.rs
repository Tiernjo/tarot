extern mod rsfml;
use rsfml::window::{event};
use rsfml::graphics::{RenderWindow};

pub fn exit(window: &mut RenderWindow) {
	loop {
		match window.poll_event() {
			event::Closed	=> window.close(),
			event::NoEvent	=> break,
			_				=> {}
		}
	}
}