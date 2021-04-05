use crossterm::event::{KeyEvent, MouseEventKind};

use line_drawing::Bresenham;

use std::convert::TryFrom;

use crate::{state::State, tools::Tool};

#[derive(Default)]
pub struct Freehand {
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

	fn render(&self) -> Vec<(usize, usize, char)> {
		self.points
			.iter()
			.copied()
			.map(|(x, y)| (x, y, '█'))
			.collect()
	}

	fn render_bounded(
		&self,
		min_x: usize,
		max_x: usize,
		min_y: usize,
		max_y: usize,
	) -> Vec<(usize, usize, char)> {
		self.points
			.iter()
			.copied()
			.filter(|(x, y)| (min_x <= *x && *x < max_x) && (min_y <= *y && *y < max_y))
			.map(|(x, y)| (x, y, '█'))
			.collect()
	}
}
