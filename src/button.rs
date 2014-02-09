extern mod rsfml;
use rsfml::graphics::{Color, RectangleShape};
use rsfml::graphics::rc::Text;
mod menu;

struct Button;

impl Button {
	fn block(&self)	-> RectangleShape {
		let block = match RectangleShape::new() {
			Some(block)	=>	block,
			None()		=>	fail!(~"Error, button block."),
		};
		block
	}
}

pub fn new(button_font:&str, button_text:&str, size_x:f32, position_x:f32, offset_x:f32, size_y:f32, position_y:f32, offset_y:f32) -> (RectangleShape, Text) {
	
	let gen_button = Button;
	let mut curr_button = gen_button.block();
	curr_button.set_size2f(size_x, size_y);
	curr_button.set_origin2f(size_x/2.0, size_y/2.0);
	curr_button.set_position2f(position_x + offset_x, position_y + offset_y);
	curr_button.set_fill_color(&Color::white());
	curr_button.set_outline_color(&Color::black());
	curr_button.set_outline_thickness(3.0);
	let curr_text = ::menu::new(button_font, button_text, 30, position_x, position_y, offset_y);
	(curr_button, curr_text)
}