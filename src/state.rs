use crossterm::{
	event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind},
	terminal::size,
};

use std::{fs::write, io::Stdout, path::PathBuf};

use crate::{
	buffer::Buffer,
	elements::{
		horizontal_scroll::HorizontalScroll, tool_menu::ToolMenu, vertical_scroll::VerticalScroll,
		workspace::Workspace, Element,
	},
	error::Result,
	tools::ToolSelect,
};

pub enum CurrentElement {
	None,
	Workspace,
	VerticalScroll,
	HorizontalScroll,
	Element(usize),
}

pub struct State {
	should_exit: bool,
	should_clear: bool,
	workspace: Workspace,
	current_mouse_element: CurrentElement,
	vertical_scroll: VerticalScroll,
	horizontal_scroll: HorizontalScroll,
	elements: Vec<Box<dyn Element>>,
	output_file: PathBuf,
}

impl State {
	pub fn new(output_file: PathBuf, load: bool) -> Result<Self> {
		let (x, y) = size().unwrap();
		let mut workspace = Workspace::new(x, y);
		if load {
			workspace.add_file_block(&output_file)?;
		}
		Ok(Self {
			should_exit: false,
			should_clear: false,
			workspace,
			current_mouse_element: CurrentElement::None,
			vertical_scroll: VerticalScroll::new(x, y),
			horizontal_scroll: HorizontalScroll::new(x, y),
			elements: vec![Box::new(ToolMenu::new(x, y))],
			output_file,
		})
	}

	pub fn should_exit(&self) -> bool { self.should_exit }

	pub fn should_clear(&self) -> bool { self.should_clear }

	pub fn set_should_clear(&mut self, should_clear: bool) { self.should_clear = should_clear; }

	pub fn reset_current_mouse_element(&mut self) {
		self.current_mouse_element = CurrentElement::None;
		self.workspace.new_tool();
	}

	pub fn set_workspace_view_offset_x(&mut self, offset: usize) {
		self.workspace.set_view_offset_x(offset);
	}

	pub fn set_workspace_view_offset_y(&mut self, offset: usize) {
		self.workspace.set_view_offset_y(offset);
	}

	pub fn set_workspace_tool(&mut self, tool: ToolSelect) {
		self.workspace.set_tool(tool);
		self.workspace.new_tool();
	}

	pub fn exit(&mut self) { self.should_exit = true }

	pub fn resize(&mut self, x: u16, y: u16) {
		self.workspace.resize_event(x, y);
		self.vertical_scroll.resize_event(x, y);
		self.horizontal_scroll.resize_event(x, y);

		for element in &mut self.elements {
			element.resize_event(x, y);
		}
		self.should_clear = true;
	}

	pub fn render(&self, w: &mut Stdout, buffer: &mut Buffer) -> Result<()> {
		self.workspace.render(w, buffer)?;
		self.vertical_scroll.render(w, buffer)?;
		self.horizontal_scroll.render(w, buffer)?;

		for element in &self.elements {
			element.render(w, buffer)?;
		}

		Ok(())
	}

	pub fn handle_event(&mut self, event: Event) -> Result<()> {
		match event {
			Event::Key(k) => {
				// Exit on ctrl-c
				if let KeyEvent {
					code: KeyCode::Char('c'),
					modifiers: KeyModifiers::CONTROL,
				} = k
				{
					self.exit()
				};
				match self.current_mouse_element {
					CurrentElement::None => match k {
						KeyEvent {
							code: KeyCode::Char('q'),
							modifiers: KeyModifiers::NONE,
						} => self.exit(),
						KeyEvent {
							code: KeyCode::Char('s'),
							modifiers: KeyModifiers::NONE,
						} => self.save_file()?,
						_ => (),
					},
					CurrentElement::Workspace => self.workspace.key_event(k)(self),
					CurrentElement::VerticalScroll => self.vertical_scroll.key_event(k)(self),
					CurrentElement::HorizontalScroll => self.horizontal_scroll.key_event(k)(self),
					CurrentElement::Element(index) => self.elements[index].key_event(k)(self),
				}
			}

			Event::Mouse(event) => {
				match self.current_mouse_element {
					CurrentElement::None => {
						if let MouseEvent {
							kind: MouseEventKind::Down(_),
							column: x,
							row: y,
							..
						} = event
						{
							// Check for mouse within workspace
							if self.workspace.coord_within(x, y) {
								// Click within workspace
								self.current_mouse_element = CurrentElement::Workspace;

								self.workspace.mouse_event(event)(self)
							}
							else if self.vertical_scroll.coord_within(x, y) {
								self.current_mouse_element = CurrentElement::VerticalScroll;

								self.workspace.mouse_event(event)(self)
							}
							else if self.horizontal_scroll.coord_within(x, y) {
								self.current_mouse_element = CurrentElement::HorizontalScroll;

								self.workspace.mouse_event(event)(self)
							}
							else {
								// Find an element with the mouse within
								if let Some((n, element)) = self
									.elements
									.iter_mut()
									.enumerate()
									.find(|(_, element)| element.coord_within(x, y))
								{
									self.current_mouse_element = CurrentElement::Element(n);

									element.mouse_event(event)(self)
								}
							}
						}
					}
					CurrentElement::Workspace => self.workspace.mouse_event(event)(self),
					CurrentElement::VerticalScroll => self.vertical_scroll.mouse_event(event)(self),
					CurrentElement::HorizontalScroll => {
						self.horizontal_scroll.mouse_event(event)(self)
					}
					CurrentElement::Element(index) => self.elements[index].mouse_event(event)(self),
				};
			}

			Event::Resize(x, y) => self.resize(x, y),
		}

		self.update_scrolls();

		Ok(())
	}

	fn update_scrolls(&mut self) {
		let ((view_start_x, view_start_y), (view_end_x, view_end_y), (max_size_x, max_size_y)) =
			self.workspace.get_parameters();
		self.horizontal_scroll
			.update_params(view_start_x, view_end_x, max_size_x);
		self.vertical_scroll
			.update_params(view_start_y, view_end_y, max_size_y);
	}

	fn save_file(&self) -> Result<()> {
		let output = self.workspace.render_to_file();
		write(&self.output_file, output.as_bytes())?;
		Ok(())
	}
}
