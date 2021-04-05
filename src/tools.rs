mod freehand;
mod none;

use crossterm::event::MouseEventKind;

use crate::{elements::buffer::Buffer, state::State};

pub trait Tool {
	fn mouse_event(
		&mut self,
		x: isize,
		y: isize,
		kind: MouseEventKind,
	) -> (fn(state: &mut State), fn(buffer: &mut Buffer));

	fn render(&self) -> Vec<(usize, usize, char)>;
}

pub enum ToolSelect {
	None,
	Freehand,
}

impl ToolSelect {
	pub fn to_tool(&self) -> Box<dyn Tool> {
		match self {
			ToolSelect::None => Box::new(none::None::default()),
			ToolSelect::Freehand => Box::new(freehand::Freehand::default()),
		}
	}
}
