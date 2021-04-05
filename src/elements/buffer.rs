use crossterm::{
	cursor::MoveTo,
	event::{MouseButton, MouseEvent, MouseEventKind},
	queue,
	style::Print,
};

use std::{convert::TryFrom, io::Stdout, iter::once, mem::replace};

use crate::{
	elements::Element,
	error::Result,
	tools::{Tool, ToolSelect},
	State,
};

pub struct Buffer {
	x: u16,
	y: u16,
	size_x: u16,
	size_y: u16,
	view_offset_x: usize,
	view_offset_y: usize,
	mouse_right_view_offset: (usize, usize),
	mouse_right_start: (u16, u16),
	current_tool_selection: ToolSelect,
	current_tool: Box<dyn Tool>,
	previous_tools: Vec<Box<dyn Tool>>,
}

impl Buffer {
	pub fn new(x: u16, y: u16) -> Self {
		let mut new = Self {
			x: 0,
			y: 0,
			size_x: 0,
			size_y: 0,
			view_offset_x: 0,
			view_offset_y: 0,
			mouse_right_view_offset: (0, 0),
			mouse_right_start: (0, 0),
			current_tool_selection: ToolSelect::Freehand,
			current_tool: ToolSelect::Freehand.to_tool(),
			previous_tools: Vec::new(),
		};
		new.resize_event(x, y);
		new
	}

	pub fn finish_tool(&mut self) {
		self.previous_tools.push(replace(
			&mut self.current_tool,
			self.current_tool_selection.to_tool(),
		))
	}

	pub fn get_parameters(&self) -> ((usize, usize), (usize, usize), (usize, usize)) {
		let mut max_x = 0;
		let mut max_y = 0;
		self.previous_tools
			.iter()
			.chain(once(&self.current_tool))
			.map(|tool| tool.render())
			.flatten()
			.for_each(|(x, y, _)| {
				max_x = max_x.max(x);
				max_y = max_y.max(y);
			});
		(
			(self.view_offset_x, self.view_offset_y),
			(
				self.view_offset_x + self.size_x as usize,
				self.view_offset_y + self.size_y as usize,
			),
			(max_x, max_y),
		)
	}
}

impl Element for Buffer {
	fn resize_event(&mut self, x: u16, y: u16) {
		self.x = 0;
		self.y = 1;
		self.size_x = x - 1;
		self.size_y = y - 2;
	}

	fn coord_within(&self, x: u16, y: u16) -> bool {
		(self.x <= x && x < self.x + self.size_x) && (self.y <= y && y < self.y + self.size_y)
	}

	fn mouse_event(
		&mut self,
		MouseEvent {
			kind,
			column: x,
			row: y,
			..
		}: MouseEvent,
	) -> fn(state: &mut State) {
		match kind {
			MouseEventKind::Down(button) => match button {
				MouseButton::Left => {
					// May rarely be out of bounds when mouse is dragged off the terminal, button let go of, and then terminal clicked on again
					if !self.coord_within(x, y) {
						return |_| ();
					}

					let global_x = self.view_offset_x as isize + x as isize - self.x as isize;
					let global_y = self.view_offset_y as isize + y as isize - self.y as isize;

					// If finished push to stack
					let (s, b) = self.current_tool.mouse_event(global_x, global_y, kind);
					b(self);
					s
				}
				MouseButton::Right => {
					self.mouse_right_start = (x, y);
					self.mouse_right_view_offset = (self.view_offset_x, self.view_offset_y);
					|_| ()
				}
				_ => |_| (),
			},
			MouseEventKind::Drag(button) => match button {
				MouseButton::Left => {
					// Ignore out of bounds drag until cursor re-enters buffer
					if !self.coord_within(x, y) {
						return |_| ();
					}

					let global_x = self.view_offset_x as isize + x as isize - self.x as isize;
					let global_y = self.view_offset_y as isize + y as isize - self.y as isize;

					// If finished push to stack
					let (s, b) = self.current_tool.mouse_event(global_x, global_y, kind);
					b(self);
					s
				}
				MouseButton::Right => {
					let (start_x, start_y) = self.mouse_right_start;

					let (start_offset_x, start_offset_y) = self.mouse_right_view_offset;

					let offset_x = start_x as isize - x as isize;

					let offset_y = start_y as isize - y as isize;

					let new_view_x = (start_offset_x as isize).saturating_add(offset_x);

					let new_view_y = (start_offset_y as isize).saturating_add(offset_y);

					self.view_offset_x = usize::try_from(new_view_x).unwrap_or(0);

					self.view_offset_y = usize::try_from(new_view_y).unwrap_or(0);

					|_| ()
				}
				_ => |_| (),
			},
			MouseEventKind::Up(button) => match button {
				MouseButton::Left => {
					let global_x = self.view_offset_x as isize + x as isize - self.x as isize;
					let global_y = self.view_offset_y as isize + y as isize - self.y as isize;

					// If finished push to stack
					let (s, b) = self.current_tool.mouse_event(global_x, global_y, kind);
					b(self);
					s
				}
				MouseButton::Right => |_| (),
				_ => |_| (),
			},
			_ => |_| (),
		}
	}

	fn render(&self, w: &mut Stdout) -> Result<()> {
		let buffer_size_x = self.size_x as usize;
		let buffer_size_y = self.size_y as usize;

		let min_x = self.view_offset_x;
		let min_y = self.view_offset_y;

		let max_x = self.view_offset_x + buffer_size_x;
		let max_y = self.view_offset_y + buffer_size_y;

		let mut buffer = vec![' '; self.size_x as usize * self.size_y as usize];

		// Write to buffer in chronological order
		self.previous_tools
			.iter()
			.chain(once(&self.current_tool))
			.map(|tool| tool.render())
			.flatten()
			.filter(|(x, y, _)| (min_x <= *x && *x < max_x) && (min_y <= *y && *y < max_y))
			.for_each(|(x, y, c)| {
				let x_in_buffer = x - self.view_offset_x;
				let y_in_buffer = y - self.view_offset_y;
				buffer[y_in_buffer * buffer_size_x + x_in_buffer] = c;
			});

		// Render buffer
		for y in 0..buffer_size_y {
			queue!(w, MoveTo(self.x, self.y + y as u16))?;
			let line = &buffer[y * buffer_size_x..(y + 1) * buffer_size_x];
			let line = line.iter().collect::<String>();
			queue!(w, Print(line))?;
		}

		Ok(())
	}
}
