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

pub fn card_change(window:&mut RenderWindow, card:&mut Sprite) {

}
