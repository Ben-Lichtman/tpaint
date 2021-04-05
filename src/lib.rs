mod elements;
mod error;
mod state;
mod tools;

use crossterm::{
	cursor::{Hide, Show},
	event::{read, DisableMouseCapture, EnableMouseCapture},
	queue,
	style::ResetColor,
	terminal::{
		disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
		LeaveAlternateScreen,
	},
};

use std::{
	env::args,
	io::{Stdout, Write},
	path::PathBuf,
};

use crate::{error::Result, state::State};

const DEFAULT_FILE_NAME: &str = "output.txt";

pub fn run(w: &mut Stdout) -> Result<()> {
	let args = args().collect::<Vec<_>>();
	let file_name = args
		.get(1)
		.map(|path| PathBuf::from(path))
		.unwrap_or(PathBuf::from(DEFAULT_FILE_NAME));

	queue!(w, EnterAlternateScreen, Hide, EnableMouseCapture)?;
	enable_raw_mode()?;

	w.flush()?;

	let mut state = State::new(file_name);

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
