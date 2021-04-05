use crossterm::event::{KeyEvent, MouseEventKind};

use std::{fs::read_to_string, path::Path};

use crate::{buffer::Buffer, error::Result, state::State, tools::Tool};

#[derive(Default)]
pub struct Block {
	chars: Vec<(usize, usize, char)>,
}

impl Block {
	pub fn new(input_file: &Path) -> Result<Self> {
		let file_str = read_to_string(input_file)?;
		let chars = file_str
			.lines()
			.enumerate()
			.flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
			.filter(|(_, _, c)| *c != ' ')
			.collect::<Vec<_>>();
		Ok(Self { chars })
	}
}

impl Tool for Block {
	fn mouse_event(&mut self, _: isize, _: isize, _: MouseEventKind) -> fn(state: &mut State) {
		|_| ()
	}

	fn key_event(&mut self, _: KeyEvent) -> fn(state: &mut State) { |_| () }

	fn bounding_box(&self) -> Option<(usize, usize, usize, usize)> {
		self.chars
			.iter()
			.copied()
			.fold(None, |acc, (x, y, _)| match acc {
				Some((min_x, max_x, min_y, max_y)) => {
					Some((min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y)))
				}
				None => Some((x, x, y, y)),
			})
	}

	fn render(&self, buffer: &mut Buffer, _: bool) {
		self.chars
			.iter()
			.copied()
			.for_each(|(x, y, c)| buffer.render_point(x, y, c))
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
		self.chars
			.iter()
			.copied()
			.filter(|(x, y, _)| (min_x <= *x && *x < max_x) && (min_y <= *y && *y < max_y))
			.for_each(|(x, y, c)| buffer.render_point(x, y, c))
	}

	fn complete(&self) -> bool { true }
}
