pub mod horizontal_scroll;
pub mod tool_menu;
pub mod vertical_scroll;
pub mod workspace;

use crossterm::event::{KeyEvent, MouseEvent};

use std::io::Stdout;

use crate::{buffer::Buffer, error::Result, state::State};

pub trait Element {
	fn resize_event(&mut self, x: u16, y: u16);

	fn coord_within(&self, x: u16, y: u16) -> bool;

	fn mouse_event(&mut self, event: MouseEvent) -> Box<dyn Fn(&mut State)>;

	fn key_event(&mut self, event: KeyEvent) -> Box<dyn Fn(&mut State)>;

	fn render(&self, w: &mut Stdout, buffer: &mut Buffer, ascii_mode: bool) -> Result<()>;
}
