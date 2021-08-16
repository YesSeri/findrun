mod finder;
mod open_error;
use clap::{App, Arg};
use finder::{File, View};
use open_error::OpenError;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), OpenError> {
	let (search_term, name) = get_args();

	let start_path = PathBuf::from(&name);
	let mut current_item = File::from(start_path.is_dir(), name, start_path);

	let mut dirs: Vec<File> = Vec::new();
	let mut results: Vec<File> = Vec::new();

	loop {
		let content = current_item.get_content().unwrap();
		for el in content.iter() {
			if el.is_dir {
				dirs.push(el.clone());
			}
		}
		for el in content {
			if el.name.to_lowercase().contains(&search_term.to_lowercase()) {
				results.push(el.clone());
			}
		}
		if dirs.is_empty() {
			break;
		}
		current_item = dirs.pop().unwrap();
	}
	let view = View::from(results.clone());
	println!("{}", view);

	loop {
		let i = match get_input() {
			Ok(res) => res,
			_ => continue,
		};
		results.get(i).unwrap().open_file().unwrap();
		break;
	}

	Ok(())
}
fn get_input() -> Result<usize, Box<dyn Error>> {
	println!("Enter number of file to open");
	let mut input = String::new();
	std::io::stdin().read_line(&mut input)?;
	let value: usize = input.trim().parse()?;
	Ok(value)
}

fn get_args() -> (String, String) {
	let matches = App::new("FindRun")
		.version("0.1")
		.author("Henrik Zenkert <henrik.zenkert@gmail.com>")
		.about("Use this to search for a program, get all similar results and choose to launch the program. ")
		// .arg(
		// 	Arg::with_name("config")
		// 		.short("c")
		// 		.long("config")
		// 		.value_name("FILE")
		// 		.help("Sets a custom config file")
		// 		.takes_value(true),
		// )
		.arg(
			Arg::with_name("SEARCH_TERM")
				.help("Sets the term you are searching for.")
				.required(true)
				.index(1),
		)
		.arg(
			Arg::with_name("LOCATION")
				.help("Sets the folder you are searching")
				.required(true)
				.index(2),
		)
		.arg(Arg::with_name("v")
				.short("v")
				.multiple(true)
				.help("Sets the level of verbosity"),
		)
		.get_matches();
	(
		matches.value_of("SEARCH_TERM").unwrap().to_string(),
		matches.value_of("LOCATION").unwrap().to_string(),
	)
}
