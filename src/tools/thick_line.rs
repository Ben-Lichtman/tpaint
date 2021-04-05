use crossterm::event::{KeyEvent, MouseEventKind};

use line_drawing::Bresenham;

use std::convert::TryFrom;

use crate::{buffer::Buffer, state::State, tools::Tool};

#[derive(Default)]
pub struct ThickLine {
	started: bool,
	start: (usize, usize),
	end: (usize, usize),
	complete: bool,
}

impl Tool for ThickLine {
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

	fn render(&self, buffer: &mut Buffer, ascii_mode: bool) {
		if !self.started {
			return;
		}

		let (start_x, start_y) = self.start;
		let (end_x, end_y) = self.end;

		for (x, y) in Bresenham::new(
			(start_x as isize, start_y as isize),
			(end_x as isize, end_y as isize),
		)
		.map(|(x, y)| (x as usize, y as usize))
		{
			buffer.render_point(x, y, if ascii_mode { '#' } else { '█' })
		}
	}

	fn render_bounded(
		&self,
		min_x: usize,
		max_x: usize,
		min_y: usize,
		max_y: usize,
		buffer: &mut Buffer,
		ascii_mode: bool,
	) {
		if !self.started {
			return;
		}

		let (start_x, start_y) = self.start;
		let (end_x, end_y) = self.end;

		for (x, y) in Bresenham::new(
			(start_x as isize, start_y as isize),
			(end_x as isize, end_y as isize),
		)
		.map(|(x, y)| (x as usize, y as usize))
		.filter(|(x, y)| (min_x <= *x && *x < max_x) && (min_y <= *y && *y < max_y))
		{
			buffer.render_point(x, y, if ascii_mode { '#' } else { '█' })
		}
	}

	fn complete(&self) -> bool { self.complete }
}
