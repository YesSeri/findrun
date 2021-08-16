use clap::{App, Arg};
pub fn get_args() -> (String, String) {
	let matches = App::new("FindRun")
		.version("0.1")
		.author("Henrik Zenkert <henrik.zenkert@gmail.com>")
		.about("Use this to search for a program, get all similar results and choose to launch the program. ")
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
