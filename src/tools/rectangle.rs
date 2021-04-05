use crossterm::event::{KeyEvent, MouseEventKind};

use std::convert::TryFrom;

use crate::{buffer::Buffer, state::State, tools::Tool};

#[derive(Default)]
pub struct Rectangle {
	started: bool,
	start: (usize, usize),
	end: (usize, usize),
	complete: bool,
}

impl Tool for Rectangle {
	fn mouse_event(&mut self, x: isize, y: isize, kind: MouseEventKind) -> fn(state: &mut State) {
		match kind {
			MouseEventKind::Down(_) => {
				if let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) {
					if !self.started {
						self.start = (x, y);
						self.end = (x, y);
						self.started = true;
						|_| ()
					}
					else {
						// Edge case - dragged off edge then released mouse
						self.end = (x, y);
						self.complete = true;
						|_| ()
					}
				}
				else {
					|_| ()
				}
			}
			MouseEventKind::Drag(_) => {
				if let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) {
					self.end = (x, y);
					self.complete = true;
				}
				|_| ()
			}
			MouseEventKind::Up(_) => |state| state.reset_current_mouse_element(),

			_ => |_| (),
		}
	}

	fn key_event(&mut self, _: KeyEvent) -> fn(state: &mut State) { |_| () }

	fn bounding_box(&self) -> Option<(usize, usize, usize, usize)> {
		if self.started {
			let (start_x, start_y) = self.start;
			let (end_x, end_y) = self.end;
			Some((
				start_x.min(end_x),
				start_x.max(end_x),
				start_y.min(end_y),
				start_y.max(end_y),
			))
		}
		else {
			None
		}
	}

	fn render(&self, buffer: &mut Buffer) {
		if !self.started {
			return;
		}

		let min_x = self.start.0.min(self.end.0);
		let max_x = self.start.0.max(self.end.0);
		let min_y = self.start.1.min(self.end.1);
		let max_y = self.start.1.max(self.end.1);
		// Top
		let top = (min_x..=max_x).map(|x| (x, min_y));
		let bottom = (min_x..=max_x).map(|x| (x, max_y));
		let left = (min_y + 1..max_y).map(|y| (min_x, y));
		let right = (min_y + 1..max_y).map(|y| (max_x, y));

		top.chain(bottom)
			.chain(left)
			.chain(right)
			.map(|(x, y)| (x, y, '█'))
			.for_each(|(x, y, c)| buffer.render_point(x, y, c))
	}

	fn render_bounded(
		&self,
		min_x: usize,
		max_x: usize,
		min_y: usize,
		max_y: usize,
		buffer: &mut Buffer,
	) {
		if !self.started {
			return;
		}

		let rect_min_x = self.start.0.min(self.end.0);
		let rect_max_x = self.start.0.max(self.end.0);
		let rect_min_y = self.start.1.min(self.end.1);
		let rect_max_y = self.start.1.max(self.end.1);
		// Top
		let top = (rect_min_x..=rect_max_x).map(|x| (x, rect_min_y));
		let bottom = (rect_min_x..=rect_max_x).map(|x| (x, rect_max_y));
		let left = (rect_min_y + 1..rect_max_y).map(|y| (rect_min_x, y));
		let right = (rect_min_y + 1..rect_max_y).map(|y| (rect_max_x, y));

		top.chain(bottom)
			.chain(left)
			.chain(right)
			.filter(|(x, y)| (min_x <= *x && *x < max_x) && (min_y <= *y && *y < max_y))
			.map(|(x, y)| (x, y, '█'))
			.for_each(|(x, y, c)| buffer.render_point(x, y, c))
	}

	fn complete(&self) -> bool { self.complete }
}
