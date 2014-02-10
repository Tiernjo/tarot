extern mod rsfml;
use rsfml::graphics::{Color, Font};
use rsfml::graphics::rc::Text;
use std::cell::RefCell;
use std::rc::Rc;

struct Menu<'s>{
	font_location:&'s str,
	contents:&'s str,
}

impl <'s>Menu<'s>{
	fn font(&self, location:&str) -> Font {
		let font = match Font::new_from_file(location) {
			Some(font)	=>	font,
			None()		=>	fail!(~"Error, menu font."),
		};
		font
	}
	fn text(&self) -> Text {
		let text = match Text::new() {
			Some(text)	=>	text,
			None()		=>	fail!(~"Error, menu text."),
		};
		text
	}
}

pub fn new(font:&str, contents:&str, size:uint, 
	window_forth_x:f32, window_half_y:f32, offset_y:f32) -> Text{
	let main:Menu = Menu{font_location:font, contents:contents};
	// Setup font
	let title_font = main.font(main.font_location);
	let title_ref_cell = RefCell::new(title_font);
	let title_rc = Rc::new(title_ref_cell);
	// Setup text
	let mut title_text = main.text();
	title_text.set_string(main.contents);
	title_text.set_character_size(size);
	title_text.set_color(&Color::black());
	title_text.set_font(title_rc);
	let text_bounds = title_text.get_local_bounds();
	title_text.set_origin2f(text_bounds.width/2.0, text_bounds.height/2.0);
	title_text.set_position2f(window_forth_x, window_half_y + offset_y);
	
	title_text
}