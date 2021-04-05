use crossterm::event::{KeyEvent, MouseEventKind};

use std::convert::TryFrom;

use crate::{box_drawing::BoxFlags, buffer::Buffer, state::State, tools::Tool};

#[derive(Default)]
pub struct Line {
	started: bool,
	start: (usize, usize),
	end: (usize, usize),
	complete: bool,
}

impl Tool for Line {
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

		let rect_min_x = self.start.0.min(self.end.0);
		let rect_max_x = self.start.0.max(self.end.0);
		let rect_min_y = self.start.1.min(self.end.1);
		let rect_max_y = self.start.1.max(self.end.1);

		if rect_max_x - rect_min_x >= rect_max_y - rect_min_y {
			// Render horizontal line
			(rect_min_x..=rect_max_x)
				.map(|x| {
					(
						x,
						start_y,
						if x == rect_min_x {
							BoxFlags::RIGHT
						}
						else if x == rect_max_x {
							BoxFlags::LEFT
						}
						else {
							BoxFlags::RIGHT | BoxFlags::LEFT
						},
					)
				})
				.for_each(|(x, y, box_dir)| {
					let current_box = BoxFlags::from_char(buffer.get_point(x, y), ascii_mode);
					let final_box = box_dir | current_box;
					buffer.render_point(x, y, final_box.to_char(ascii_mode))
				})
		}
		else {
			// Render vertical line
			(rect_min_y..=rect_max_y)
				.map(|y| {
					(
						start_x,
						y,
						if y == rect_min_y {
							BoxFlags::DOWN
						}
						else if y == rect_max_y {
							BoxFlags::UP
						}
						else {
							BoxFlags::UP | BoxFlags::DOWN
						},
					)
				})
				.for_each(|(x, y, box_dir)| {
					let current_box = BoxFlags::from_char(buffer.get_point(x, y), ascii_mode);
					let final_box = box_dir | current_box;
					buffer.render_point(x, y, final_box.to_char(ascii_mode))
				})
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

		let rect_min_x = self.start.0.min(self.end.0);
		let rect_max_x = self.start.0.max(self.end.0);
		let rect_min_y = self.start.1.min(self.end.1);
		let rect_max_y = self.start.1.max(self.end.1);

		if rect_max_x - rect_min_x >= rect_max_y - rect_min_y {
			// Render horizontal line
			(rect_min_x..=rect_max_x)
				.map(|x| {
					(
						x,
						start_y,
						if x == rect_min_x {
							BoxFlags::RIGHT
						}
						else if x == rect_max_x {
							BoxFlags::LEFT
						}
						else {
							BoxFlags::RIGHT | BoxFlags::LEFT
						},
					)
				})
				.filter(|(x, y, _)| (min_x <= *x && *x < max_x) && (min_y <= *y && *y < max_y))
				.for_each(|(x, y, box_dir)| {
					let current_box = BoxFlags::from_char(buffer.get_point(x, y), ascii_mode);
					let final_box = box_dir | current_box;
					buffer.render_point(x, y, final_box.to_char(ascii_mode))
				})
		}
		else {
			// Render vertical line
			(rect_min_y..=rect_max_y)
				.map(|y| {
					(
						start_x,
						y,
						if y == rect_min_y {
							BoxFlags::DOWN
						}
						else if y == rect_max_y {
							BoxFlags::UP
						}
						else {
							BoxFlags::UP | BoxFlags::DOWN
						},
					)
				})
				.filter(|(x, y, _)| (min_x <= *x && *x < max_x) && (min_y <= *y && *y < max_y))
				.for_each(|(x, y, box_dir)| {
					let current_box = BoxFlags::from_char(buffer.get_point(x, y), ascii_mode);
					let final_box = box_dir | current_box;
					buffer.render_point(x, y, final_box.to_char(ascii_mode))
				})
		}
	}

	fn complete(&self) -> bool { self.complete }
}
