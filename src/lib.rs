use crossterm::{
	cursor::{Hide, Show},
	event::{read, DisableMouseCapture, EnableMouseCapture},
	queue,
};

use crossterm::{
	style::ResetColor,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io::Write;

mod buffer;
mod elements;
mod error;
mod state;
mod tool;

use crate::{error::Error, state::State};

pub fn run(w: &mut impl Write) -> Result<(), Error> {
	queue!(w, EnterAlternateScreen, Hide, EnableMouseCapture)?;
	enable_raw_mode()?;

	w.flush()?;

	let mut state = State::new();

	while state.should_exit() == false {
		state.render(w)?;

		w.flush()?;

		state.handle_event(w, read()?)?;
	}

	queue!(
		w,
		ResetColor,
		DisableMouseCapture,
		Show,
		LeaveAlternateScreen
	)?;
	disable_raw_mode()?;

	Ok(())
}
