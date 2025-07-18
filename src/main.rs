use rand::{SeedableRng, seq::IteratorRandom};
use std::{
	fs::File,
	io::{BufRead, BufReader},
};
use terminal_size::terminal_size;

fn main() {
	let mut args = pico_args::Arguments::from_env();
	if args.contains(["-h", "--help"]) {
		print_help();
	}

	let path = dirs::home_dir().unwrap().join(".prayers");
	let file = File::open(path).expect("a .prayers file in ~ is expected and not found");

	let b = BufReader::new(file);
	let mut rng = rand::rngs::SmallRng::from_os_rng();

	let width = terminal_size().unwrap().0.0;

	let mut line = b.lines().choose(&mut rng).unwrap().unwrap();

	if args.contains("--rev") {
		line = line.chars().rev().collect();
	}
	println!("{line:>0$}", width as usize);
}

fn print_help() {
	println!("Print a dua, read from $HOME/.prayers\n");
	println!("OPTIONS:");
	println!(
		"{:>6} {:<6}\treverse the text. useful for e.g. Ghostty",
		"", "--rev"
	);
	println!("{:>6} {:<6}\tprint this message", "-h", "--help");
}
