use std::io::Write;

use crate::buffer::Buffer;
use crate::error::Error;

pub enum Tool {
	None,
	Pen,
	Erase,
	Text,
}

impl Tool {
	pub fn tool_name(&self) -> &'static str {
		match self {
			Tool::None => "None",
			Tool::Pen => "Pen",
			Tool::Erase => "Erase",
			Tool::Text => "Text",
		}
	}

	pub fn left_mouse_down_event(
		&self,
		buffer: &mut Buffer,
		x: u16,
		y: u16,
		view_offset_x: usize,
		view_offset_y: usize,
	) -> Result<(), Error> {
		match self {
			Tool::None => (),
			Tool::Pen => buffer.write(
				'█',
				view_offset_x + x as usize,
				view_offset_y + y as usize,
				false,
			),
			Tool::Erase => buffer.write(
				' ',
				view_offset_x + x as usize,
				view_offset_y + y as usize,
				true,
			),
			Tool::Text => todo!(),
		}
		Ok(())
	}

	pub fn left_mouse_up_event(&self) -> Result<(), Error> {
		match self {
			Tool::None => (),
			Tool::Pen => (),
			Tool::Erase => (),
			Tool::Text => (),
		}
		Ok(())
	}

	pub fn left_mouse_drag_event(
		&self,
		buffer: &mut Buffer,
		x: u16,
		y: u16,
		view_offset_x: usize,
		view_offset_y: usize,
	) -> Result<(), Error> {
		match self {
			Tool::None => (),
			Tool::Pen => buffer.write(
				'█',
				view_offset_x + x as usize,
				view_offset_y + y as usize,
				false,
			),
			Tool::Erase => buffer.write(
				' ',
				view_offset_x + x as usize,
				view_offset_y + y as usize,
				true,
			),
			Tool::Text => todo!(),
		}
		Ok(())
	}
}
