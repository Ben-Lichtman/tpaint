use crossterm::event::MouseEventKind;

use crate::{elements::buffer::Buffer, state::State, tools::Tool};

#[derive(Default)]
pub struct None;

impl Tool for None {
	fn mouse_event(
		&mut self,
		x: isize,
		y: isize,
		kind: MouseEventKind,
	) -> (fn(state: &mut State), fn(buffer: &mut Buffer)) {
		(|_| (), |_| ())
	}

	fn render(&self) -> Vec<(usize, usize, char)> { Vec::new() }

	fn render_bounded(&self, _: usize, _: usize, _: usize, _: usize) -> Vec<(usize, usize, char)> {
		Vec::new()
	}
}
