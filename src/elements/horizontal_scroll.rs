use crossterm::{cursor::MoveTo, event::MouseEvent, queue, style::Print};

use std::io::Stdout;

use crate::{elements::Element, error::Result, State};

pub struct HorizontalScroll {
	x: u16,
	y: u16,
	length: u16,
	view_start: usize,
	view_end: usize,
	max_size: usize,
}

impl HorizontalScroll {
	pub fn new(x: u16, y: u16) -> Self {
		let mut new = Self {
			x: 0,
			y: 0,
			length: 0,
			view_start: 0,
			view_end: 0,
			max_size: 0,
		};
		new.resize_event(x, y);
		new
	}

	pub fn update_params(&mut self, view_start: usize, view_end: usize, max_size: usize) {
		self.view_start = view_start;
		self.view_end = view_end;
		self.max_size = max_size;
	}
}

impl Element for HorizontalScroll {
	fn resize_event(&mut self, x: u16, y: u16) {
		self.x = 1;
		self.y = y - 1;
		self.length = x - 2;
		self.max_size = self.length as usize;
	}

	fn coord_within(&self, x: u16, y: u16) -> bool {
		(self.x <= x && x < self.x + self.length) && y == self.y
	}

	fn mouse_event(&mut self, event: MouseEvent) -> fn(state: &mut State) { |_| () }

	fn render(&self, w: &mut Stdout) -> Result<()> {
		let max_size = self.max_size.max(self.view_end);
		let view_start_bar = ((self.length as usize * self.view_start) / max_size) as u16;
		let view_end_bar = ((self.length as usize * self.view_end) / max_size) as u16;

		for offset in 0..self.length {
			queue!(w, MoveTo(self.x + offset, self.y))?;

			if offset < view_start_bar {
				queue!(w, Print('░'))?;
			}
			else if offset < view_end_bar {
				queue!(w, Print('▓'))?;
			}
			else {
				queue!(w, Print('░'))?;
			}
		}

		Ok(())
	}
}
