use crossterm::event::{KeyCode, KeyEvent, MouseEventKind};

use std::{convert::TryFrom, iter::once};

use crate::{elements::buffer::Buffer, state::State, tools::Tool};

#[derive(Default)]
pub struct Text {
	x: usize,
	y: usize,
	text: String,
	in_progress: bool,
}

impl Tool for Text {
	fn mouse_event(
		&mut self,
		x: isize,
		y: isize,
		kind: MouseEventKind,
	) -> (fn(state: &mut State), fn(buffer: &mut Buffer)) {
		match kind {
			MouseEventKind::Down(_) => {
				if !self.in_progress {
					if let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) {
						self.x = x;
						self.y = y;
						self.in_progress = true;
					}
					(|_| (), |_| ())
				}
				else {
					self.in_progress = false;
					(
						|state| state.reset_current_mouse_element(),
						|buffer| buffer.finish_tool(),
					)
				}
			}
			_ => (|_| (), |_| ()),
		}
	}

	fn key_event(&mut self, event: KeyEvent) -> (fn(state: &mut State), fn(buffer: &mut Buffer)) {
		match event {
			KeyEvent {
				code: KeyCode::Esc,
				modifiers: _,
			} => {
				self.in_progress = false;
				(
					|state| state.reset_current_mouse_element(),
					|buffer| buffer.finish_tool(),
				)
			}
			KeyEvent {
				code: KeyCode::Enter,
				modifiers: _,
			} => {
				self.in_progress = false;
				(
					|state| state.reset_current_mouse_element(),
					|buffer| buffer.finish_tool(),
				)
			}
			KeyEvent {
				code: KeyCode::Char(c),
				modifiers: _,
			} => {
				self.text.push(c);
				(|_| (), |_| ())
			}
			_ => (|_| (), |_| ()),
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
