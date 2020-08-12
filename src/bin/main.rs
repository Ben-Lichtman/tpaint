use std::io::stdout;

use tui_app::run;

fn main() {
	let mut stdout = stdout();

	run(&mut stdout).unwrap();
}
