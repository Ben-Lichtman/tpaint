use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{size, Clear, ClearType};

use std::borrow::Cow;
use std::convert::TryFrom;
use std::io::Write;

use crate::buffer::Buffer;
use crate::elements::{BarElement, ButtonBar, ScrollBar, ToolBar};
use crate::error::Error;
use crate::tool::Tool;

pub struct State {
	should_exit: bool,
	size_x: u16,
	size_y: u16,
	view_offset_x: usize,
	view_offset_y: usize,
	view_offset_temp_x: isize,
	view_offset_temp_y: isize,
	button_bar: ButtonBar,
	tool_bar: ToolBar,
	bottom_scroll: ScrollBar,
	side_scroll: ScrollBar,
	buffer: Buffer,
	current_tool: Tool,
	mouse_left_start: Option<(u16, u16, KeyModifiers)>,
	mouse_right_start: Option<(u16, u16, KeyModifiers)>,
}

fn tool_pen(state: &mut State) -> Result<(), Error> {
	state.current_tool = Tool::Pen;
	Ok(())
}

fn tool_erase(state: &mut State) -> Result<(), Error> {
	state.current_tool = Tool::Erase;
	Ok(())
}

impl State {
	pub fn new() -> Self {
		let (size_x, size_y) = size().unwrap();

		let default_buttons = vec![
			BarElement::Text(Cow::Borrowed("Tpaint")),
			BarElement::Text(Cow::Borrowed("    ")),
			BarElement::Button(Cow::Borrowed("⚫"), tool_pen),
			BarElement::Text(Cow::Borrowed("    ")),
			BarElement::Button(Cow::Borrowed("⚪"), tool_erase),
			BarElement::Text(Cow::Borrowed("    ")),
		];

		Self {
			should_exit: false,
			size_x,
			size_y,
			view_offset_x: 0,
			view_offset_y: 0,
			view_offset_temp_x: 0,
			view_offset_temp_y: 0,
			button_bar: ButtonBar::new(default_buttons),
			tool_bar: ToolBar,
			bottom_scroll: ScrollBar { vertical: false },
			side_scroll: ScrollBar { vertical: true },
			buffer: Buffer::new(),
			current_tool: Tool::None,
			mouse_left_start: None,
			mouse_right_start: None,
		}
	}

	pub fn should_exit(&self) -> bool { self.should_exit }

	pub fn render(&self, w: &mut impl Write) -> Result<(), Error> {
		queue!(w, MoveTo(0, 0))?;

		let (view_offset_x, view_offset_y) = self.view_offset();

		self.button_bar.render(w)?;

		self.tool_bar.render(w, &self.current_tool)?;

		self.buffer.render(
			w,
			view_offset_x,
			view_offset_y,
			self.size_x - 1,
			self.size_y - 4,
		)?;

		queue!(w, Clear(ClearType::UntilNewLine), MoveToNextLine(1))?;

		self.bottom_scroll.render(
			w,
			view_offset_x,
			self.size_x - 1,
			self.buffer.max_dimensions().0,
			self.size_x - 1,
		)?;

		queue!(w, MoveTo(self.size_x - 1, 2))?;

		self.side_scroll.render(
			w,
			view_offset_y,
			self.size_y - 4,
			self.buffer.max_dimensions().1,
			self.size_y - 4,
		)?;

		w.flush()?;

		Ok(())
	}

	pub fn handle_event(&mut self, w: &mut impl Write, event: Event) -> Result<(), Error> {
		match event {
			Event::Key(k) => match k {
				KeyEvent {
					code: KeyCode::Char('c'),
					modifiers: KeyModifiers::CONTROL,
				} => self.should_exit = true,

				_ => (),
			},

			Event::Mouse(m) => match m {
				MouseEvent::Down(button, x, y, modifier) => match button {
					MouseButton::Left => {
						self.left_mouse_down_event(button, x, y, modifier)?;
						self.mouse_left_start = Some((x, y, modifier))
					}
					MouseButton::Right => {
						self.right_mouse_down_event(button, x, y, modifier)?;
						self.mouse_right_start = Some((x, y, modifier))
					}
					MouseButton::Middle => (),
				},

				MouseEvent::Up(button, x, y, modifier) => match button {
					MouseButton::Left => {
						self.left_mouse_up_event(button, x, y, modifier)?;
						self.mouse_left_start = None
					}
					MouseButton::Right => {
						self.right_mouse_up_event(button, x, y, modifier)?;
						self.mouse_right_start = None
					}
					MouseButton::Middle => (),
				},

				MouseEvent::Drag(button, x, y, modifier) => match button {
					MouseButton::Left => {
						self.left_mouse_drag_event(button, x, y, modifier)?;
					}
					MouseButton::Right => {
						self.right_mouse_drag_event(button, x, y, modifier)?;
					}
					MouseButton::Middle => (),
				},

				_ => (),
			},

			Event::Resize(x, y) => {
				self.size_x = x;
				self.size_y = y;
			}
		}

		Ok(())
	}

	fn left_mouse_down_event(
		&mut self,
		button: MouseButton,
		x: u16,
		y: u16,
		modifier: KeyModifiers,
	) -> Result<(), Error> {
		let (view_offset_x, view_offset_y) = self.view_offset();
		if x == self.size_x - 1 {
		}
		else if y == 0 {
			let button = self.button_bar.get(x);
			if let Some(BarElement::Button(_, f)) = button {
				f(self)?;
			}
		}
		else if y == 1 {
		}
		else if y < self.size_y - 2 {
			let y = y - 2;
			self.current_tool.left_mouse_down_event(
				&mut self.buffer,
				x,
				y,
				view_offset_x,
				view_offset_y,
			)?;
		}
		else if y == self.size_y - 1 {
		}
		Ok(())
	}

	fn left_mouse_up_event(
		&mut self,
		button: MouseButton,
		x: u16,
		y: u16,
		modifier: KeyModifiers,
	) -> Result<(), Error> {
		let (prev_x, prev_y, prev_modifier) = self.mouse_left_start.unwrap();
		Ok(())
	}

	fn left_mouse_drag_event(
		&mut self,
		button: MouseButton,
		x: u16,
		y: u16,
		modifier: KeyModifiers,
	) -> Result<(), Error> {
		let (prev_x, prev_y, prev_modifier) = self.mouse_left_start.unwrap();
		let (view_offset_x, view_offset_y) = self.view_offset();
		if x == self.size_x - 1 {
		}
		else if y == 0 {
		}
		else if y == 1 {
		}
		else if y < self.size_y - 2 {
			let y = y - 2;
			self.current_tool.left_mouse_drag_event(
				&mut self.buffer,
				x,
				y,
				view_offset_x,
				view_offset_y,
			)?;
		}
		else if y == self.size_y - 1 {
		}
		Ok(())
	}

	fn right_mouse_down_event(
		&mut self,
		button: MouseButton,
		x: u16,
		y: u16,
		modifier: KeyModifiers,
	) -> Result<(), Error> {
		Ok(())
	}

	fn right_mouse_up_event(
		&mut self,
		button: MouseButton,
		x: u16,
		y: u16,
		modifier: KeyModifiers,
	) -> Result<(), Error> {
		let (prev_x, prev_y, prev_modifier) = self.mouse_right_start.unwrap();
		let x = (self.view_offset_x as isize).saturating_add(self.view_offset_temp_x);
		let y = (self.view_offset_y as isize).saturating_add(self.view_offset_temp_y);
		let x = usize::try_from(x).unwrap_or(0);
		let y = usize::try_from(y).unwrap_or(0);
		self.view_offset_x = x;
		self.view_offset_y = y;
		self.view_offset_temp_x = 0;
		self.view_offset_temp_y = 0;
		Ok(())
	}

	fn right_mouse_drag_event(
		&mut self,
		button: MouseButton,
		x: u16,
		y: u16,
		modifier: KeyModifiers,
	) -> Result<(), Error> {
		let (prev_x, prev_y, prev_modifier) = self.mouse_right_start.unwrap();
		self.view_offset_temp_x = prev_x as isize - x as isize;
		self.view_offset_temp_y = prev_y as isize - y as isize;
		Ok(())
	}

	fn view_offset(&self) -> (usize, usize) {
		let x = (self.view_offset_x as isize).saturating_add(self.view_offset_temp_x);
		let y = (self.view_offset_y as isize).saturating_add(self.view_offset_temp_y);
		let x = usize::try_from(x).unwrap_or(0);
		let y = usize::try_from(y).unwrap_or(0);
		(x, y)
	}
}
