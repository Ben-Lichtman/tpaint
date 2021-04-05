use crossterm::event::{KeyEvent, MouseEventKind};

use line_drawing::Bresenham;

use std::convert::TryFrom;

use crate::{buffer::Buffer, state::State, tools::Tool};

#[derive(Default)]
pub struct Freehand {
	started: bool,
	points: Vec<(usize, usize)>,
}

impl Tool for Freehand {
	fn mouse_event(&mut self, x: isize, y: isize, kind: MouseEventKind) -> fn(state: &mut State) {
		if let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) {
			if let Some((old_x, old_y)) = self.points.last() {
				for (x, y) in
					Bresenham::new((*old_x as isize, *old_y as isize), (x as isize, y as isize))
						.skip(1)
				{
					self.points.push((x as usize, y as usize));
				}
			}
			else {
				self.started = true;
				self.points.push((x, y));
			}
		}

		// Finish tool when mouse releases
		if let MouseEventKind::Up(_) = kind {
			return |state| state.reset_current_mouse_element();
		}

		|_| ()
	}

	fn key_event(&mut self, _: KeyEvent) -> fn(state: &mut State) { |_| () }

	fn bounding_box(&self) -> Option<(usize, usize, usize, usize)> {
		self.points
			.iter()
			.copied()
			.fold(None, |acc, (x, y)| match acc {
				Some((min_x, max_x, min_y, max_y)) => {
					Some((min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y)))
				}
				None => Some((x, x, y, y)),
			})
	}

	fn render(&self, buffer: &mut Buffer, ascii_mode: bool) {
		self.points
			.iter()
			.copied()
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
		self.points
			.iter()
			.copied()
			.filter(|(x, y)| (min_x <= *x && *x < max_x) && (min_y <= *y && *y < max_y))
			.map(|(x, y)| (x, y, '█'))
			.for_each(|(x, y, c)| buffer.render_point(x, y, c))
	}

	fn complete(&self) -> bool { self.started }
}
