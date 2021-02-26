use std::env;
use std::fs;
use std::format;
use std::process;

fn usage() {
	let program_name =
		env::current_exe()
		.ok().expect("")
		.file_name().expect("")
		.to_str().expect("")
		.to_owned();
	eprintln!("Usage: {} [command] [args...]", program_name);
	process::exit(1);
}

fn get_content(filename : &str) -> String {
	fs::read_to_string(filename).expect(&format!("Failed to read {}.", filename))
}

// print [filename]
fn cmd_print(args : &[String]) {
	if args.len() < 1 {
		usage();
	}

	let filename = &args[0];
	let content = get_content(&filename);

	let mut ln = 0;
	for line in content.lines() {
		println!("{}: {}", ln, line);
		ln += 1;
	}
}

// search [pattern] [filename]
fn cmd_search(args : &[String]) {
	if args.len() < 2 {
		usage();
	}

	let pattern = &args[0];
	let filename = &args[1];
	let content = get_content(&filename);

	let mut ln = 0;
	for line in content.lines() {
		if line.contains(pattern) {
			println!("{}: {}", ln, line);
		}
		ln += 1;
	}
}

fn main() {
	let args : Vec<String> = env::args().collect();

	if args.len() < 2 {
		usage();
	} else {
		let cmd_name = args[1].as_str();
		let cmd_args = &args[2..];

		match cmd_name {
			"print" => cmd_print(cmd_args),
			"search" => cmd_search(cmd_args),
			_ => usage(),
		};
	}
}