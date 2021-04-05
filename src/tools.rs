mod freehand;
mod none;
mod rectangle;
mod text;

use crossterm::event::{KeyEvent, MouseEventKind};

use crate::{elements::buffer::Buffer, state::State};

pub trait Tool {
	fn mouse_event(
		&mut self,
		x: isize,
		y: isize,
		kind: MouseEventKind,
	) -> (fn(state: &mut State), fn(buffer: &mut Buffer));

	fn key_event(&mut self, event: KeyEvent) -> (fn(state: &mut State), fn(buffer: &mut Buffer));

	fn render(&self) -> Vec<(usize, usize, char)>;

	fn render_bounded(
		&self,
		min_x: usize,
		max_x: usize,
		min_y: usize,
		max_y: usize,
	) -> Vec<(usize, usize, char)>;
}

#[derive(Clone, Copy)]
pub enum ToolSelect {
	None,
	Freehand,
	Rectangle,
	Text,
}

impl ToolSelect {
	pub fn to_tool(&self) -> Box<dyn Tool> {
		match self {
			ToolSelect::None => Box::new(none::None::default()),
			ToolSelect::Freehand => Box::new(freehand::Freehand::default()),
			ToolSelect::Rectangle => Box::new(rectangle::Rectangle::default()),
			ToolSelect::Text => Box::new(text::Text::default()),
		}
	}

	pub fn name(&self) -> &'static str {
		match self {
			ToolSelect::None => "None",
			ToolSelect::Freehand => "Freehand",
			ToolSelect::Rectangle => "Rectangle",
			ToolSelect::Text => "Text",
		}
	}
}