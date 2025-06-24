use rand::{SeedableRng, seq::IteratorRandom};
use std::{
	fs::File,
	io::{BufRead, BufReader},
};
use terminal_size::terminal_size;

fn main() {
	let path = dirs::home_dir().unwrap().join(".prayers");
	let file = File::open(path).expect("a .prayers file in ~ is expected and not found");

	let b = BufReader::new(file);
	let mut rng = rand::rngs::SmallRng::from_os_rng();

	let width = terminal_size().unwrap().0.0;

	let mut line = b.lines().choose(&mut rng).unwrap().unwrap();

	let mut args = pico_args::Arguments::from_env();
	if args.contains("--rev") {
		line = line.chars().rev().collect();
	}
	println!("{line:>0$}", width as usize);
}
