use crossterm::event::{KeyEvent, MouseEventKind};

use bitflags::bitflags;

use std::{convert::TryFrom, iter::once};

use crate::{buffer::Buffer, state::State, tools::Tool};

#[derive(Default)]
pub struct Rectangle {
	started: bool,
	start: (usize, usize),
	end: (usize, usize),
	complete: bool,
}

bitflags! {
	struct BoxDir: u8 {
		const NONE = 0b0000;
		const UP = 0b0001;
		const DOWN = 0b0010;
		const LEFT = 0b0100;
		const RIGHT = 0b1000;
	}
}

impl BoxDir {
	fn from_char(c: char) -> Self {
		match c {
			'╋' => Self::UP | Self::DOWN | Self::LEFT | Self::RIGHT,

			'┳' => Self::DOWN | Self::LEFT | Self::RIGHT,
			'┻' => Self::UP | Self::LEFT | Self::RIGHT,
			'┣' => Self::UP | Self::DOWN | Self::RIGHT,
			'┫' => Self::UP | Self::DOWN | Self::LEFT,

			'┛' => Self::UP | Self::LEFT,
			'┗' => Self::UP | Self::RIGHT,
			'┓' => Self::DOWN | Self::LEFT,
			'┏' => Self::DOWN | Self::RIGHT,

			'┃' => Self::UP | Self::DOWN,
			'━' => Self::LEFT | Self::RIGHT,

			_ => Self::NONE,
		}
	}

	fn to_char(self) -> char {
		match (
			self.contains(Self::UP),
			self.contains(Self::DOWN),
			self.contains(Self::LEFT),
			self.contains(Self::RIGHT),
		) {
			(true, true, true, true) => '╋',

			(false, true, true, true) => '┳',
			(true, false, true, true) => '┻',
			(true, true, false, true) => '┣',
			(true, true, true, false) => '┫',

			(true, false, true, false) => '┛',
			(true, false, false, true) => '┗',
			(false, true, true, false) => '┓',
			(false, true, false, true) => '┏',

			(true, true, false, false) => '┃',
			(false, false, true, true) => '━',

			_ => ' ',
		}
	}
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

		let top_left = (rect_min_x, rect_min_y, BoxDir::DOWN | BoxDir::RIGHT);
		let top_right = (rect_max_x, rect_min_y, BoxDir::DOWN | BoxDir::LEFT);
		let bottom_left = (rect_min_x, rect_max_y, BoxDir::UP | BoxDir::RIGHT);
		let bottom_right = (rect_max_x, rect_max_y, BoxDir::UP | BoxDir::LEFT);

		top.map(|(x, y)| (x, y, BoxDir::LEFT | BoxDir::RIGHT))
			.chain(bottom.map(|(x, y)| (x, y, BoxDir::LEFT | BoxDir::RIGHT)))
			.chain(left.map(|(x, y)| (x, y, BoxDir::UP | BoxDir::DOWN)))
			.chain(right.map(|(x, y)| (x, y, BoxDir::UP | BoxDir::DOWN)))
			.chain(once(top_left))
			.chain(once(top_right))
			.chain(once(bottom_left))
			.chain(once(bottom_right))
			.for_each(|(x, y, box_dir)| {
				let current_box = BoxDir::from_char(buffer.get_point(x, y));
				let final_box = box_dir | current_box;
				buffer.render_point(x, y, final_box.to_char())
			})
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

		let top = (rect_min_x + 1..rect_max_x).map(|x| (x, rect_min_y));
		let bottom = (rect_min_x + 1..rect_max_x).map(|x| (x, rect_max_y));
		let left = (rect_min_y + 1..rect_max_y).map(|y| (rect_min_x, y));
		let right = (rect_min_y + 1..rect_max_y).map(|y| (rect_max_x, y));

		let top_left = (rect_min_x, rect_min_y, BoxDir::DOWN | BoxDir::RIGHT);
		let top_right = (rect_max_x, rect_min_y, BoxDir::DOWN | BoxDir::LEFT);
		let bottom_left = (rect_min_x, rect_max_y, BoxDir::UP | BoxDir::RIGHT);
		let bottom_right = (rect_max_x, rect_max_y, BoxDir::UP | BoxDir::LEFT);

		top.map(|(x, y)| (x, y, BoxDir::LEFT | BoxDir::RIGHT))
			.chain(bottom.map(|(x, y)| (x, y, BoxDir::LEFT | BoxDir::RIGHT)))
			.chain(left.map(|(x, y)| (x, y, BoxDir::UP | BoxDir::DOWN)))
			.chain(right.map(|(x, y)| (x, y, BoxDir::UP | BoxDir::DOWN)))
			.chain(once(top_left))
			.chain(once(top_right))
			.chain(once(bottom_left))
			.chain(once(bottom_right))
			.filter(|(x, y, _)| (min_x <= *x && *x < max_x) && (min_y <= *y && *y < max_y))
			.for_each(|(x, y, box_dir)| {
				let current_box = BoxDir::from_char(buffer.get_point(x, y));
				let final_box = box_dir | current_box;
				buffer.render_point(x, y, final_box.to_char())
			})
	}

	fn complete(&self) -> bool { self.complete }
}
