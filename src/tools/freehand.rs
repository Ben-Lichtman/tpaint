use crossterm::event::{KeyEvent, MouseEventKind};

use std::convert::TryFrom;

use crate::{elements::buffer::Buffer, state::State, tools::Tool};

#[derive(Default)]
pub struct Freehand {
	points: Vec<(usize, usize)>,
}

impl Tool for Freehand {
	fn mouse_event(
		&mut self,
		x: isize,
		y: isize,
		kind: MouseEventKind,
	) -> (fn(state: &mut State), fn(buffer: &mut Buffer)) {
		if let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) {
			self.points.push((x, y));
		}

		// Finish tool when mouse releases
		if let MouseEventKind::Up(_) = kind {
			(
				|state| state.reset_current_mouse_element(),
				|buffer| buffer.finish_tool(),
			)
		}
		else {
			(|_| (), |_| ())
		}
	}

	fn key_event(&mut self, event: KeyEvent) -> (fn(state: &mut State), fn(buffer: &mut Buffer)) {
		(|_| (), |_| ())
	}

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
