use crossterm::event::{KeyEvent, MouseEventKind};

use std::{convert::TryFrom, iter::once};

use crate::{box_drawing::BoxFlags, buffer::Buffer, state::State, tools::Tool};

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

	fn render(&self, buffer: &mut Buffer, ascii_mode: bool) {
		if !self.started {
			return;
		}

		let rect_min_x = self.start.0.min(self.end.0);
		let rect_max_x = self.start.0.max(self.end.0);
		let rect_min_y = self.start.1.min(self.end.1);
		let rect_max_y = self.start.1.max(self.end.1);

		let top = (rect_min_x + 1..rect_max_x).map(|x| (x, rect_min_y));
		let bottom = (rect_min_x + 1..rect_max_x).map(|x| (x, rect_max_y));
		let left = (rect_min_y + 1..rect_max_y).map(|y| (rect_min_x, y));
		let right = (rect_min_y + 1..rect_max_y).map(|y| (rect_max_x, y));

		let top_left = (rect_min_x, rect_min_y, BoxFlags::DOWN | BoxFlags::RIGHT);
		let top_right = (rect_max_x, rect_min_y, BoxFlags::DOWN | BoxFlags::LEFT);
		let bottom_left = (rect_min_x, rect_max_y, BoxFlags::UP | BoxFlags::RIGHT);
		let bottom_right = (rect_max_x, rect_max_y, BoxFlags::UP | BoxFlags::LEFT);

		top.map(|(x, y)| (x, y, BoxFlags::LEFT | BoxFlags::RIGHT))
			.chain(bottom.map(|(x, y)| (x, y, BoxFlags::LEFT | BoxFlags::RIGHT)))
			.chain(left.map(|(x, y)| (x, y, BoxFlags::UP | BoxFlags::DOWN)))
			.chain(right.map(|(x, y)| (x, y, BoxFlags::UP | BoxFlags::DOWN)))
			.chain(once(top_left))
			.chain(once(top_right))
			.chain(once(bottom_left))
			.chain(once(bottom_right))
			.for_each(|(x, y, box_dir)| {
				let current_box = BoxFlags::from_char(buffer.get_point(x, y), ascii_mode);
				let final_box = box_dir | current_box;
				buffer.render_point(x, y, final_box.to_char(ascii_mode))
			})
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

		let rect_min_x = self.start.0.min(self.end.0);
		let rect_max_x = self.start.0.max(self.end.0);
		let rect_min_y = self.start.1.min(self.end.1);
		let rect_max_y = self.start.1.max(self.end.1);

		let top = (rect_min_x + 1..rect_max_x).map(|x| (x, rect_min_y));
		let bottom = (rect_min_x + 1..rect_max_x).map(|x| (x, rect_max_y));
		let left = (rect_min_y + 1..rect_max_y).map(|y| (rect_min_x, y));
		let right = (rect_min_y + 1..rect_max_y).map(|y| (rect_max_x, y));

		let top_left = (rect_min_x, rect_min_y, BoxFlags::DOWN | BoxFlags::RIGHT);
		let top_right = (rect_max_x, rect_min_y, BoxFlags::DOWN | BoxFlags::LEFT);
		let bottom_left = (rect_min_x, rect_max_y, BoxFlags::UP | BoxFlags::RIGHT);
		let bottom_right = (rect_max_x, rect_max_y, BoxFlags::UP | BoxFlags::LEFT);

		top.map(|(x, y)| (x, y, BoxFlags::LEFT | BoxFlags::RIGHT))
			.chain(bottom.map(|(x, y)| (x, y, BoxFlags::LEFT | BoxFlags::RIGHT)))
			.chain(left.map(|(x, y)| (x, y, BoxFlags::UP | BoxFlags::DOWN)))
			.chain(right.map(|(x, y)| (x, y, BoxFlags::UP | BoxFlags::DOWN)))
			.chain(once(top_left))
			.chain(once(top_right))
			.chain(once(bottom_left))
			.chain(once(bottom_right))
			.filter(|(x, y, _)| (min_x <= *x && *x < max_x) && (min_y <= *y && *y < max_y))
			.for_each(|(x, y, box_dir)| {
				let current_box = BoxFlags::from_char(buffer.get_point(x, y), ascii_mode);
				let final_box = box_dir | current_box;
				buffer.render_point(x, y, final_box.to_char(ascii_mode))
			})
	}

	fn complete(&self) -> bool { self.complete }
}
