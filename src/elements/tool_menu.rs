use crossterm::{
	cursor::MoveTo,
	event::{KeyEvent, MouseButton, MouseEvent, MouseEventKind},
	queue,
	style::Print,
	terminal::{Clear, ClearType},
};

use unicode_width::UnicodeWidthStr;

use std::io::Stdout;

use crate::{elements::Element, error::Result, tools::ToolSelect, State};

enum MenuElement {
	Divider,
	Text(&'static str),
	Tool(&'static str, ToolSelect),
}

impl MenuElement {
	fn width(&self) -> usize {
		match self {
			Self::Divider => 3,
			Self::Text(t) => UnicodeWidthStr::width(*t),
			Self::Tool(t, _) => UnicodeWidthStr::width(*t),
		}
	}

	fn render(&self, w: &mut Stdout) -> Result<()> {
		match self {
			Self::Divider => queue!(w, Print(" | "))?,
			Self::Text(t) => queue!(w, Print(t))?,
			Self::Tool(t, _) => queue!(w, Print(t))?,
		}
		Ok(())
	}
}

pub struct ToolMenu {
	x: u16,
	y: u16,
	length: u16,
	elements: Vec<MenuElement>,
	selected: ToolSelect,
}

impl ToolMenu {
	pub fn new(x: u16, y: u16) -> Self {
		let mut new = Self {
			x: 0,
			y: 0,
			length: 0,
			elements: vec![
				MenuElement::Tool("⚫", ToolSelect::Freehand),
				MenuElement::Divider,
				MenuElement::Tool("[]", ToolSelect::Rectangle),
				MenuElement::Divider,
				MenuElement::Tool("T", ToolSelect::Text),
			],
			selected: ToolSelect::None,
		};
		new.resize_event(x, y);
		new
	}
}

impl Element for ToolMenu {
	fn resize_event(&mut self, x: u16, y: u16) {
		self.x = 0;
		self.y = 0;
		self.length = x - 1;
	}

	fn coord_within(&self, x: u16, y: u16) -> bool {
		(self.x <= x && x < self.x + self.length) && self.y == y
	}

	fn mouse_event(
		&mut self,
		MouseEvent {
			kind, column: x, ..
		}: MouseEvent,
	) -> Box<dyn Fn(&mut State)> {
		match kind {
			MouseEventKind::Down(button) => match button {
				MouseButton::Left => {
					let offset = x - self.x;

					let mut counter = 0;
					for element in &self.elements {
						let width = element.width();
						if counter <= (offset as usize) && (offset as usize) < counter + width {
							match element {
								MenuElement::Tool(_, tool) => {
									let tool = *tool;
									self.selected = tool;
									return Box::new(move |state| {
										state.reset_current_mouse_element();
										state.set_buffer_tool(tool);
									});
								}
								_ => (),
							}
						}
						counter += width;
					}

					Box::new(|state| state.reset_current_mouse_element())
				}
				_ => Box::new(|state| state.reset_current_mouse_element()),
			},
			_ => Box::new(|state| state.reset_current_mouse_element()),
		}
	}

	fn key_event(&mut self, event: KeyEvent) -> Box<dyn Fn(&mut State)> { Box::new(|_| ()) }

	fn render(&self, w: &mut Stdout) -> Result<()> {
		queue!(w, MoveTo(self.x, self.y))?;
		self.elements
			.iter()
			.map(|e| e.render(w))
			.collect::<Result<Vec<_>>>()?;
		queue!(w, MoveTo(self.x, self.y + 1))?;
		queue!(w, Print(self.selected.name()))?;
		queue!(w, Clear(ClearType::UntilNewLine))?;
		Ok(())
	}
}