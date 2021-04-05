use std::iter::once;

pub struct Buffer {
	start_x: usize,
	start_y: usize,
	grid: Vec<Vec<char>>,
}

impl Buffer {
	pub fn new() -> Self {
		Self {
			start_x: 0,
			start_y: 0,
			grid: Vec::new(),
		}
	}

	pub fn new_frame(&mut self) {
		self.start_x = 0;
		self.start_y = 0;
		self.grid.clear();
	}

	pub fn new_frame_bounded(&mut self, start_x: usize, start_y: usize) {
		self.start_x = start_x;
		self.start_y = start_y;
		self.grid.clear();
	}

	pub fn get_point(&self, x: usize, y: usize) -> char {
		*self.grid.get(y).and_then(|y| y.get(x)).unwrap_or(&' ')
	}

	pub fn render_point(&mut self, x: usize, y: usize, c: char) {
		let x = x - self.start_x;
		let y = y - self.start_y;

		if self.grid.len() < y + 1 {
			self.grid.resize(y + 1, Vec::new());
		}
		let row = &mut self.grid[y];
		if row.len() < x + 1 {
			row.resize(x + 1, ' ');
		}
		row[x] = c;
	}

	pub fn into_inner(self) -> Vec<Vec<char>> { self.grid }

	pub fn output(&self, width: usize) -> Vec<String> {
		self.grid
			.iter()
			.map(|line| {
				line.iter()
					.copied()
					.chain(once(' ').cycle())
					.take(width)
					.collect::<String>()
			})
			.collect::<Vec<_>>()
	}
}
