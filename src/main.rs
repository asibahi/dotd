use rand::{SeedableRng, seq::IteratorRandom};
use std::{
	fs::File,
	io::{BufRead, BufReader},
};
use terminal_size::{Width, terminal_size};

fn main() {
	let mut args = pico_args::Arguments::from_env();
	if args.contains(["-h", "--help"]) {
		print_help();
		std::process::exit(0)
	}

	let Some((Width(width), _)) = terminal_size() else {
		failure("Not a terminal");
	};
	let Some(file) = dirs::home_dir()
		.map(|p| p.join(".prayers"))
		.and_then(|p| File::open(p).inspect_err(|e| eprintln!("{e}")).ok())
	else {
		failure("A .prayers file in the home directory is expected and not found");
	};

	let b = BufReader::new(file);
	let mut rng = rand::rngs::SmallRng::from_os_rng();

	let Some(Ok(mut line)) = b.lines().choose(&mut rng) else {
		failure("The .prayers file must not be empty")
	};

	if args.contains("--rev") || args.contains("--reverse") {
		line = line.chars().rev().collect();
	}
	if args.contains("--rev-words") {
		line = line.split_whitespace().rev().map(|s| format!("{s} ")).collect();
	}
	println!("{line:>0$}", width as usize);
}

fn print_help() {
	let s = 3;
	let l = 9;

	println!("Print a dua, read from $HOME/.prayers\n");
	println!("OPTIONS:");
	println!("{:>s$} {:<l$}\treverse the whole text. useful for e.g. Ghostty", "", "--rev");
	println!(
		"{:>s$} {:<l$}\treverse the words order. useful for some terminals",
		"", "--rev-words"
	);
	println!("{:>s$} {:<l$}\tprint this message", "-h", "--help");
}

fn failure(msg: &str) -> ! {
	eprintln!("{msg}");
	std::process::exit(1)
}
