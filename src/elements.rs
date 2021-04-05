pub mod buffer;
pub mod horizontal_scroll;
pub mod vertical_scroll;

use crossterm::event::MouseEvent;

use std::io::Stdout;

use crate::{error::Result, state::State};

pub trait Element {
	fn resize_event(&mut self, x: u16, y: u16);

	fn coord_within(&self, x: u16, y: u16) -> bool;

	fn mouse_event(&mut self, event: MouseEvent) -> fn(state: &mut State);

	fn render(&self, w: &mut Stdout) -> Result<()>;
}
