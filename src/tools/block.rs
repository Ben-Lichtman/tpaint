use crossterm::event::{KeyEvent, MouseEventKind};

use std::{fs::read_to_string, path::Path};

use crate::{elements::buffer::Buffer, error::Result, state::State, tools::Tool};

#[derive(Default)]
pub struct Block {
	lines: Vec<String>,
}

impl Block {
	pub fn new(input_file: &Path) -> Result<Self> {
		let file_str = read_to_string(input_file)?;
		let lines = file_str
			.lines()
			.map(|line| line.to_string())
			.collect::<Vec<_>>();
		Ok(Self { lines })
	}
}

impl Tool for Block {
	fn mouse_event(
		&mut self,
		x: isize,
		y: isize,
		kind: MouseEventKind,
	) -> (fn(state: &mut State), fn(buffer: &mut Buffer)) {
		(|_| (), |_| ())
	}

	fn key_event(&mut self, event: KeyEvent) -> (fn(state: &mut State), fn(buffer: &mut Buffer)) {
		(|_| (), |_| ())
	}

	fn render(&self) -> Vec<(usize, usize, char)> {
		self.lines
			.iter()
			.enumerate()
			.flat_map(|(y, line)| line.chars().enumerate().map(move |(x, char)| (x, y, char)))
			.collect::<Vec<_>>()
	}

	fn render_bounded(
		&self,
		min_x: usize,
		max_x: usize,
		min_y: usize,
		max_y: usize,
	) -> Vec<(usize, usize, char)> {
		self.lines
			.iter()
			.enumerate()
			.filter(|(y, _)| min_y <= *y && *y < max_y)
			.flat_map(|(y, line)| {
				line.chars()
					.enumerate()
					.filter(|(x, _)| min_x <= *x && *x < max_x)
					.map(move |(x, char)| (x, y, char))
			})
			.collect::<Vec<_>>()
	}
}
