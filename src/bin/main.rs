use std::io::stdout;

use tpaint::run;

fn main() {
	let mut stdout = stdout();

	run(&mut stdout).unwrap();
}
