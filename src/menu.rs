extern mod rsfml;
use rsfml::graphics::{Color, Font, Text};

struct Menu<'s>{
	font_location:&'s str,
	contents:&'s str,
}

impl Menu {
	fn font(&self, location:&str) -> Font {
		let font = match Font::new_from_file(location) {
			Some(font)	=>	font,
			None()		=>	fail!(~"Error, menu font."),
		};
		font
	}
	fn text(&self, contents:&str, font:&Font, size:uint) -> Text {
		let text = match Text::new_init(contents, font, size) {
			Some(text)	=>	text,
			None()		=>	fail!(~"Error, menu text."),
		};
		text
	}
}

pub fn new(){
	let main = Menu{font_location:"../resources/font/Jura-Regular.ttf", contents:"Welcome to my Tarot app!"};
	let title_font = main.font(main.font_location);
	let title_text = main.text(main.contents, &title_font, 30);
	title_text.set_color(&Color::black());
	title_text
}