use crossterm::{
	cursor::MoveTo,
	event::{KeyEvent, MouseButton, MouseEvent, MouseEventKind},
	queue,
	style::Print,
};

use std::{convert::TryFrom, io::Stdout, iter::once, path::Path};

use crate::{
	buffer::Buffer,
	elements::Element,
	error::Result,
	tools::{block::Block, Tool, ToolSelect},
	State,
};

pub struct Workspace {
	x: u16,
	y: u16,
	size_x: u16,
	size_y: u16,
	view_offset_x: usize,
	view_offset_y: usize,
	mouse_right_view_offset: (usize, usize),
	mouse_right_start: (u16, u16),
	current_tool_selection: ToolSelect,
	previous_tools: Vec<Box<dyn Tool>>,
}

impl Workspace {
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
			current_tool_selection: ToolSelect::None,
			previous_tools: vec![ToolSelect::None.to_tool()],
		};
		new.resize_event(x, y);
		new
	}

	pub fn get_parameters(&self) -> ((usize, usize), (usize, usize), (usize, usize)) {
		let (_, max_x, _, max_y) = self
			.previous_tools
			.iter()
			.map(|tool| tool.bounding_box())
			.fold(
				None,
				|acc: Option<(usize, usize, usize, usize)>, new| match (acc, new) {
					(
						Some((min_x, max_x, min_y, max_y)),
						Some((new_min_x, new_max_x, new_min_y, new_max_y)),
					) => Some((
						min_x.min(new_min_x),
						max_x.max(new_max_x),
						min_y.min(new_min_y),
						max_y.max(new_max_y),
					)),
					(Some(old), None) => Some(old),
					(None, Some(new)) => Some(new),
					(None, None) => None,
				},
			)
			.unwrap_or((0, 0, 0, 0));
		(
			(self.view_offset_x, self.view_offset_y),
			(
				self.view_offset_x + self.size_x as usize,
				self.view_offset_y + self.size_y as usize,
			),
			(max_x, max_y),
		)
	}

	pub fn set_view_offset_x(&mut self, offset: usize) { self.view_offset_x = offset }

	pub fn set_view_offset_y(&mut self, offset: usize) { self.view_offset_y = offset }

	pub fn new_tool(&mut self) {
		if let Some(last) = self.previous_tools.last() {
			if !last.complete() {
				self.previous_tools.pop();
			}
		}
		self.previous_tools
			.push(self.current_tool_selection.to_tool());
	}

	pub fn set_tool(&mut self, tool: ToolSelect) { self.current_tool_selection = tool; }

	pub fn render_to_file(&self) -> String {
		let mut buffer = Buffer::new();

		// Write to buffer in chronological order
		self.previous_tools
			.iter()
			.for_each(|tool| tool.render(&mut buffer));

		// Convert each line to String and write out to file
		buffer
			.into_inner()
			.into_iter()
			.flat_map(|line| line.into_iter().chain(once('\n')))
			.collect::<String>()
	}

	pub fn add_file_block(&mut self, input_file: &Path) -> Result<()> {
		self.previous_tools.push(Box::new(Block::new(input_file)?));
		Ok(())
	}

	pub fn undo(&mut self) {
		self.previous_tools.pop();
		self.previous_tools.pop();
		self.new_tool();
	}
}

impl Element for Workspace {
	fn resize_event(&mut self, x: u16, y: u16) {
		self.x = 0;
		self.y = 2;
		self.size_x = x - 1;
		self.size_y = y - 3;
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
	) -> Box<dyn Fn(&mut State)> {
		match kind {
			MouseEventKind::Down(button) => match button {
				MouseButton::Left => {
					// May rarely be out of bounds when mouse is dragged off the terminal, button let go of, and then terminal clicked on again
					if !self.coord_within(x, y) {
						return Box::new(|_| ());
					}

					let global_x = self.view_offset_x as isize + x as isize - self.x as isize;
					let global_y = self.view_offset_y as isize + y as isize - self.y as isize;

					let current_tool = self.previous_tools.last_mut().unwrap();

					Box::new(current_tool.mouse_event(global_x, global_y, kind))
				}
				MouseButton::Right => {
					self.mouse_right_start = (x, y);
					self.mouse_right_view_offset = (self.view_offset_x, self.view_offset_y);
					Box::new(|_| ())
				}
				_ => Box::new(|_| ()),
			},
			MouseEventKind::Drag(button) => match button {
				MouseButton::Left => {
					// Ignore out of bounds drag until cursor re-enters buffer
					if !self.coord_within(x, y) {
						return Box::new(|_| ());
					}

					let global_x = self.view_offset_x as isize + x as isize - self.x as isize;
					let global_y = self.view_offset_y as isize + y as isize - self.y as isize;

					let current_tool = self.previous_tools.last_mut().unwrap();

					Box::new(current_tool.mouse_event(global_x, global_y, kind))
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

					Box::new(|_| ())
				}
				_ => Box::new(|_| ()),
			},
			MouseEventKind::Up(button) => match button {
				MouseButton::Left => {
					let global_x = self.view_offset_x as isize + x as isize - self.x as isize;
					let global_y = self.view_offset_y as isize + y as isize - self.y as isize;

					let current_tool = self.previous_tools.last_mut().unwrap();

					Box::new(current_tool.mouse_event(global_x, global_y, kind))
				}
				MouseButton::Right => Box::new(|_| ()),
				_ => Box::new(|_| ()),
			},
			_ => Box::new(|_| ()),
		}
	}

	fn key_event(&mut self, event: KeyEvent) -> Box<dyn Fn(&mut State)> {
		let current_tool = self.previous_tools.last_mut().unwrap();

		Box::new(current_tool.key_event(event))
	}

	fn render(&self, w: &mut Stdout, buffer: &mut Buffer) -> Result<()> {
		let buffer_size_x = self.size_x as usize;
		let buffer_size_y = self.size_y as usize;

		let min_x = self.view_offset_x;
		let min_y = self.view_offset_y;

		let max_x = self.view_offset_x + buffer_size_x;
		let max_y = self.view_offset_y + buffer_size_y;

		buffer.new_frame_bounded(min_x, min_y);

		// Write to buffer in chronological order
		self.previous_tools
			.iter()
			.for_each(|tool| tool.render_bounded(min_x, max_x, min_y, max_y, buffer));

		let lines = buffer.output(buffer_size_x);

		// Render buffer
		for y in 0..buffer_size_y {
			queue!(w, MoveTo(self.x, self.y + y as u16))?;
			if let Some(line) = lines.get(y) {
				queue!(w, Print(line))?;
			}
			else {
				let line = once(' ').cycle().take(buffer_size_x).collect::<String>();
				queue!(w, Print(line))?;
			}
		}

		Ok(())
	}
}
