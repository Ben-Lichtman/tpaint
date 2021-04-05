use crossterm::event::{KeyCode, KeyEvent, MouseEventKind};

use std::convert::TryFrom;

use crate::{buffer::Buffer, state::State, tools::Tool};

#[derive(Default)]
pub struct Text {
	x: usize,
	y: usize,
	text: Vec<String>,
	in_progress: bool,
	finished: bool,
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
					self.finished = true;

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
				self.finished = true;
				|state| state.reset_current_mouse_element()
			}
			KeyEvent {
				code: KeyCode::Backspace,
				modifiers: _,
			} => {
				if let Some(line) = self.text.last_mut() {
					if line.len() == 0 {
						self.text.pop();
					}
					else {
						line.pop();
					}
				}
				|_| ()
			}
			KeyEvent {
				code: KeyCode::Enter,
				modifiers: _,
			} => {
				self.text.push(String::new());
				|_| ()
			}
			KeyEvent {
				code: KeyCode::Char(c),
				modifiers: _,
			} => {
				if let Some(line) = self.text.last_mut() {
					line.push(c);
				}
				else {
					self.text.push(String::from(c))
				}

				|_| ()
			}
			_ => |_| (),
		}
	}

	fn bounding_box(&self) -> Option<(usize, usize, usize, usize)> {
		let longest_line = self.text.iter().map(|line| line.len()).max().unwrap_or(0);
		Some((
			self.x,
			self.x + longest_line,
			self.y,
			self.y + self.text.len(),
		))
	}

	fn render(&self, buffer: &mut Buffer, _: bool) {
		self.text.iter().enumerate().for_each(|(y, line)| {
			line.chars()
				.enumerate()
				.map(|(x, c)| (self.x + x, self.y + y, c))
				.for_each(|(x, y, c)| buffer.render_point(x, y, c));
		});
		if self.in_progress {
			buffer.render_point(
				self.x + self.text.last().map(|l| l.len()).unwrap_or(0),
				self.y + self.text.len().saturating_sub(1),
				'<',
			)
		}
	}

	fn render_bounded(
		&self,
		min_x: usize,
		max_x: usize,
		min_y: usize,
		max_y: usize,
		buffer: &mut Buffer,
		_: bool,
	) {
		self.text.iter().enumerate().for_each(|(y, line)| {
			line.chars()
				.enumerate()
				.map(|(x, c)| (self.x + x, self.y + y, c))
				.filter(|(x, y, _)| (min_x <= *x && *x < max_x) && (min_y <= *y && *y < max_y))
				.for_each(|(x, y, c)| buffer.render_point(x, y, c));
		});
		if self.in_progress {
			buffer.render_point(
				self.x + self.text.last().map(|l| l.len()).unwrap_or(0),
				self.y + self.text.len().saturating_sub(1),
				'<',
			)
		}
	}

	fn complete(&self) -> bool { self.finished }
}
