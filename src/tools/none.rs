use crossterm::event::{KeyEvent, MouseEventKind};

use crate::{buffer::Buffer, state::State, tools::Tool};

#[derive(Default)]
pub struct None;

impl Tool for None {
	fn mouse_event(&mut self, _: isize, _: isize, _: MouseEventKind) -> fn(state: &mut State) {
		|state| state.reset_current_mouse_element()
	}

	fn key_event(&mut self, _: KeyEvent) -> fn(state: &mut State) { |_| () }

	fn bounding_box(&self) -> Option<(usize, usize, usize, usize)> { Option::None }

	fn render(&self, _: &mut Buffer) {}

	fn render_bounded(&self, _: usize, _: usize, _: usize, _: usize, _: &mut Buffer) {}

	fn complete(&self) -> bool { false }
}
