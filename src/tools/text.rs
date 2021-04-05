use crossterm::event::{KeyCode, KeyEvent, MouseEventKind};

use std::convert::TryFrom;

use crate::{state::State, tools::Tool};

#[derive(Default)]
pub struct Text {
	x: usize,
	y: usize,
	text: String,
	in_progress: bool,
}

impl Tool for Text {
	fn mouse_event(&mut self, x: isize, y: isize, kind: MouseEventKind) -> fn(state: &mut State) {
		match kind {
			MouseEventKind::Down(_) => {
				if !self.in_progress {
					if let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) {
						self.x = x;
						self.y = y;
						self.in_progress = true;
					}
					|_| ()
				}
				else {
					self.in_progress = false;

					|state| state.reset_current_mouse_element()
				}
			}
			_ => |_| (),
		}
	}

	fn key_event(&mut self, event: KeyEvent) -> fn(state: &mut State) {
		match event {
			KeyEvent {
				code: KeyCode::Esc,
				modifiers: _,
			} => {
				self.in_progress = false;
				|state| state.reset_current_mouse_element()
			}
			KeyEvent {
				code: KeyCode::Backspace,
				modifiers: _,
			} => {
				self.text.pop();
				|_| ()
			}
			KeyEvent {
				code: KeyCode::Enter,
				modifiers: _,
			} => {
				self.in_progress = false;

				|state| state.reset_current_mouse_element()
			}
			KeyEvent {
				code: KeyCode::Char(c),
				modifiers: _,
			} => {
				self.text.push(c);
				|_| ()
			}
			_ => |_| (),
		}
	}

	fn render(&self) -> Vec<(usize, usize, char)> {
		self.text
			.chars()
			.enumerate()
			.map(|(n, c)| (self.x + n, self.y, c))
			.collect::<Vec<_>>()
	}

	fn render_bounded(
		&self,
		min_x: usize,
		max_x: usize,
		min_y: usize,
		max_y: usize,
	) -> Vec<(usize, usize, char)> {
		if min_y <= self.y && self.y < max_y {
			self.text
				.chars()
				.chain(if self.in_progress { Some('<') } else { None })
				.enumerate()
				.filter(|(n, _)| min_x <= self.x + n && self.x + n < max_x)
				.map(|(n, c)| (self.x + n, self.y, c))
				.collect::<Vec<_>>()
		}
		else {
			Vec::new()
		}
	}
}
