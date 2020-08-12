use crossterm::cursor::{MoveDown, RestorePosition, SavePosition};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};

use std::io::Write;

use crate::error::Error;

pub struct Buffer {
	max_width: usize,
	max_height: usize,
	data: Vec<Vec<char>>,
}

impl Buffer {
	pub fn new() -> Self {
		Self {
			max_width: 0,
			max_height: 0,
			data: Vec::new(),
		}
	}

	pub fn render(
		&self,
		w: &mut impl Write,
		offset_x: usize,
		offset_y: usize,
		width: u16,
		height: u16,
	) -> Result<(), Error> {
		for y in 0..height {
			queue!(w, SavePosition)?;

			let mut line = String::with_capacity(width as usize);

			if let Some(row) = self.data.get(offset_y + y as usize) {
				for x in 0..width {
					if let Some(c) = row.get(offset_x + x as usize) {
						line.push(*c);
					}
					else {
						line.push(' ');
					}
				}
			}
			else {
				for _ in 0..width {
					line.push(' ');
				}
			}

			queue!(w, Print(line))?;
			// queue!(w, Clear(ClearType::UntilNewLine))?;
			queue!(w, RestorePosition, MoveDown(1))?;
		}

		Ok(())
	}

	pub fn write(&mut self, c: char, x: usize, y: usize, on_top: bool) {
		if self.data.len() < y + 1 {
			self.data.resize(y + 1, Vec::new());
			if y + 1 > self.max_height {
				self.max_height = y + 1;
			}
		}
		let row = &mut self.data[y];
		if row.len() < x + 1 {
			row.resize(x + 1, ' ');
			if x + 1 > self.max_height {
				self.max_width = x + 1;
			}
		}
		if on_top {
			self.data[y][x] = c;
		}
		else if self.data[y][x] == ' ' {
			self.data[y][x] = c;
		}
	}

	pub fn max_dimensions(&self) -> (usize, usize) { (self.max_width, self.max_height) }
}
