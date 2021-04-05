use crossterm::{
	event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind},
	terminal::size,
};

use std::io::Stdout;

use crate::{
	elements::{
		buffer::Buffer, horizontal_scroll::HorizontalScroll, vertical_scroll::VerticalScroll,
		Element,
	},
	error::Result,
};

pub enum CurrentElement {
	None,
	Buffer,
	VerticalScroll,
	HorizontalScroll,
	Element(usize),
}

pub struct State {
	should_exit: bool,
	should_clear: bool,
	buffer: Buffer,
	vertical_scroll: VerticalScroll,
	horizontal_scroll: HorizontalScroll,
	current_mouse_element: CurrentElement,
	elements: Vec<Box<dyn Element>>,
}

impl State {
	pub fn new() -> Self {
		let (x, y) = size().unwrap();
		Self {
			should_exit: false,
			should_clear: false,
			buffer: Buffer::new(x, y),
			vertical_scroll: VerticalScroll::new(x, y),
			horizontal_scroll: HorizontalScroll::new(x, y),
			current_mouse_element: CurrentElement::None,
			elements: vec![],
		}
	}

	pub fn should_exit(&self) -> bool { self.should_exit }

	pub fn should_clear(&self) -> bool { self.should_clear }

	pub fn set_should_clear(&mut self, should_clear: bool) { self.should_clear = should_clear; }

	pub fn reset_current_mouse_element(&mut self) {
		self.current_mouse_element = CurrentElement::None;
	}

	pub fn exit(&mut self) { self.should_exit = true }

	pub fn resize(&mut self, x: u16, y: u16) {
		self.buffer.resize_event(x, y);
		self.vertical_scroll.resize_event(x, y);
		self.horizontal_scroll.resize_event(x, y);

		for element in &mut self.elements {
			element.resize_event(x, y);
		}
		self.should_clear = true;
	}

	pub fn render(&self, w: &mut Stdout) -> Result<()> {
		self.buffer.render(w)?;
		self.vertical_scroll.render(w)?;
		self.horizontal_scroll.render(w)?;
		for element in &self.elements {
			element.render(w)?;
		}
		Ok(())
	}

	pub fn handle_event(&mut self, event: Event) -> Result<()> {
		match event {
			Event::Key(k) => match k {
				KeyEvent {
					code: KeyCode::Char('c'),
					modifiers: KeyModifiers::CONTROL,
				} => self.exit(),
				KeyEvent {
					code: KeyCode::Char('q'),
					modifiers: KeyModifiers::NONE,
				} => self.exit(),
				_ => (),
			},

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
							// Check for mouse within buffer
							if self.buffer.coord_within(x, y) {
								// Click within buffer
								self.current_mouse_element = CurrentElement::Buffer;

								self.buffer.mouse_event(event)(self)
							}
							else if self.vertical_scroll.coord_within(x, y) {
								self.current_mouse_element = CurrentElement::VerticalScroll;

								self.buffer.mouse_event(event)(self)
							}
							else if self.horizontal_scroll.coord_within(x, y) {
								self.current_mouse_element = CurrentElement::HorizontalScroll;

								self.buffer.mouse_event(event)(self)
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
					CurrentElement::Buffer => self.buffer.mouse_event(event)(self),
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
			self.buffer.get_parameters();
		self.horizontal_scroll
			.update_params(view_start_x, view_end_x, max_size_x);
		self.vertical_scroll
			.update_params(view_start_y, view_end_y, max_size_y);
	}
}
