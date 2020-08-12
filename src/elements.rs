use crossterm::cursor::{MoveDown, MoveToNextLine};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};

use unicode_width::UnicodeWidthStr;

use std::borrow::Cow;
use std::cmp::max;
use std::io::Write;

use crate::error::Error;
use crate::state::State;

pub enum BarElement {
	Divider,
	Text(Cow<'static, str>),
	Button(Cow<'static, str>, fn(&mut State) -> Result<(), Error>),
}

impl BarElement {
	pub fn width(&self) -> usize {
		match self {
			BarElement::Divider => 1,
			BarElement::Text(t) => UnicodeWidthStr::width(t.as_ref()),
			BarElement::Button(t, _) => UnicodeWidthStr::width(t.as_ref()),
		}
	}

	pub fn render(&self, w: &mut impl Write) -> Result<(), Error> {
		match self {
			BarElement::Divider => queue!(w, Print("|"))?,
			BarElement::Text(t) => queue!(w, Print(t))?,
			BarElement::Button(t, _) => queue!(w, Print(t))?,
		}

		Ok(())
	}
}

pub struct ButtonBar(Vec<BarElement>);

impl ButtonBar {
	pub fn new(elements: Vec<BarElement>) -> Self { Self(elements) }

	pub fn render(&self, w: &mut impl Write) -> Result<(), Error> {
		for element in &self.0 {
			element.render(w)?;
		}
		queue!(w, Clear(ClearType::UntilNewLine))?;
		queue!(w, MoveToNextLine(1))?;
		Ok(())
	}

	pub fn get(&self, offset: u16) -> Option<&BarElement> {
		let mut counter = 0;
		for element in &self.0 {
			let width = element.width();
			if counter <= (offset as usize) && (offset as usize) < counter + width {
				return Some(element);
			}
			counter += width;
		}

		None
	}

	pub fn get_mut(&mut self, offset: u16) -> Option<&mut BarElement> {
		let mut counter = 0;
		for element in &mut self.0 {
			let width = element.width();
			if counter <= (offset as usize) && (offset as usize) < counter + width {
				return Some(element);
			}

			counter += width;
		}

		None
	}

	pub fn set(&mut self, offset: u16, element: BarElement) { self.0[offset as usize] = element; }
}

pub struct ScrollBar {
	pub vertical: bool,
}

impl ScrollBar {
	pub fn render(
		&self,
		w: &mut impl Write,
		view_offset: usize,
		view_size: u16,
		max_size: usize,
		bar_size: u16,
	) -> Result<(), Error> {
		let view_start = view_offset;
		let view_end = view_offset + view_size as usize;
		let max_size = max(view_end, max_size);

		let view_start_bar = (bar_size as usize * view_start) / max_size;
		let view_end_bar = (bar_size as usize * view_end) / max_size;

		if self.vertical {
			for _ in 0..view_start_bar {
				queue!(w, Print("░"))?;
				queue!(w, MoveDown(1))?;
			}
			for _ in view_start_bar..view_end_bar {
				queue!(w, Print("▓"))?;
				queue!(w, MoveDown(1))?;
			}
			for _ in view_end_bar..bar_size as usize {
				queue!(w, Print("░"))?;
				queue!(w, MoveDown(1))?;
			}
		}
		else {
			for _ in 0..view_start_bar {
				queue!(w, Print("░"))?;
			}
			for _ in view_start_bar..view_end_bar {
				queue!(w, Print("▓"))?;
			}
			for _ in view_end_bar..bar_size as usize {
				queue!(w, Print("░"))?;
			}
		}

		Ok(())
	}
}
