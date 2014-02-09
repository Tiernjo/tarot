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
pub fn button(window: &mut RenderWindow, button_bounds:FloatRect) -> uint {
	let mut screen:uint = 1;
	let button_origin_x = button_bounds.left as i32;
	let button_origin_y = button_bounds.height as i32*4;
	let button_end_x = button_bounds.width as i32;
	let button_end_y = button_bounds.top as i32+56;
	let mut mouse_loc = window.get_mouse_position();
	println!("button_bounds.top is {}\nbutton_bounds.left is {}", button_bounds.top, button_bounds.left);
	println!("mouse_loc.x is {}\nmouse_loc.y is {}", mouse_loc.x, mouse_loc.y);
	println!("button_origin_x is {}\nbutton_end_y is {}", button_origin_x, button_end_y);
	println!("button_end_x is {}\nbutton_origin_y is {}", button_end_x, button_origin_y);

	//if (mouse_loc.x >= button_origin_x) && (mouse_loc.x <= button_end_x) && mouse::is_button_pressed(mouse::MouseLeft){screen = 2}
	if (mouse_loc.y >= button_origin_y) && (mouse_loc.y <= button_end_y) && mouse::is_button_pressed(mouse::MouseLeft){screen = 2}
	if keyboard::is_key_pressed(keyboard::Escape) {window.close()}
	screen
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
