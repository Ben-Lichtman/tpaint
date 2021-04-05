mod elements;
mod error;
mod state;
mod tools;

use crossterm::{
	cursor::{Hide, Show},
	event::{read, DisableMouseCapture, EnableMouseCapture, MouseEvent},
	queue,
	style::ResetColor,
	terminal::{
		disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
		LeaveAlternateScreen,
	},
};

use std::io::{Stdout, Write};

use crate::{error::Result, state::State};

pub fn run(w: &mut Stdout) -> Result<()> {
	queue!(w, EnterAlternateScreen, Hide, EnableMouseCapture)?;
	enable_raw_mode()?;

	w.flush()?;

	let mut state = State::new();

	while state.should_exit() == false {
		if state.should_clear() {
			queue!(w, Clear(ClearType::All))?;
			state.set_should_clear(false);
		}

		state.render(w)?;

		w.flush()?;

		state.handle_event(read()?)?;
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
