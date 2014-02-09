extern mod rsfml;
use rsfml::window::{event, keyboard, mouse};
use rsfml::graphics::{RenderWindow, FloatRect};
use rsfml::graphics::rc::{Sprite};
use rsfml::system::{Vector2i};

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
	if keyboard::is_key_pressed(keyboard::I){screen = 4}
	screen
}
pub fn button(window: &mut RenderWindow, button_bounds:FloatRect, to_screen:uint) -> (uint, bool) {
	let mut screen:uint = 1; let mut is_clicked = false;
	let button_origin_x = button_bounds.left as i32;
	let button_end_y = button_bounds.height as i32+200+30;
	let button_end_x = button_bounds.width as i32 + 150;
	let button_origin_y = button_bounds.top as i32+56;
	let mut mouse_loc = window.get_mouse_position();
	// MouseX is more than box x, MouseX is less than width
	// MouseY is more than box top, MouseY is less than height
	if (mouse_loc.x >= button_origin_x) && (mouse_loc.x <= button_end_x) 
	&& (mouse_loc.y >= button_end_y) && (mouse_loc.y <= button_origin_y) 
	&& mouse::is_button_pressed(mouse::MouseLeft){screen = to_screen;is_clicked = true}
	
	(screen, is_clicked)
}

pub fn card_shift(window:&mut RenderWindow) -> (int, uint) {
	let mut move = 0; let mut screen:uint = 2;

	if keyboard::is_key_pressed(keyboard::D){move = 1}
	if keyboard::is_key_pressed(keyboard::A){move = -1}
	if keyboard::is_key_pressed(keyboard::Escape){screen = 5}
	(move, screen)
}

pub fn reading(mut screen:uint) -> uint {
	if keyboard::is_key_pressed(keyboard::Escape){screen = 5}
	screen
}
