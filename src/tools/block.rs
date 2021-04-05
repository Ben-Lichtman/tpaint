use crossterm::event::{KeyEvent, MouseEventKind};

use std::{fs::read_to_string, path::Path};

use crate::{error::Result, state::State, tools::Tool};

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
	fn mouse_event(&mut self, x: isize, y: isize, kind: MouseEventKind) -> fn(state: &mut State) {
		|_| ()
	}

	fn key_event(&mut self, event: KeyEvent) -> fn(state: &mut State) { |_| () }

	fn render(&self) -> Vec<(usize, usize, char)> { self.chars.clone() }

	fn render_bounded(
		&self,
		min_x: usize,
		max_x: usize,
		min_y: usize,
		max_y: usize,
	) -> Vec<(usize, usize, char)> {
		self.chars
			.iter()
			.copied()
			.filter(|(x, y, _)| (min_x <= *x && *x < max_x) && (min_y <= *y && *y < max_y))
			.collect()
	}
}
