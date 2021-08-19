mod args_parser;
// use args_parser::get_args;
mod model;
use model::Finder;
mod quality_control;
use quality_control::Timer;
use std::sync::{Arc, Mutex};
mod view;
use std::thread;
use view::View;

fn main() {
	// let (search_phrase, location) = get_args();
	let (search_phrase, location) = (
		String::from(".pdf"),
		// std::path::PathBuf::from("C:/Programming"),
		std::path::PathBuf::from("C:/Games"),
	);
	let results = Arc::new(Mutex::new(vec![]));
	let res_clone = Arc::clone(&results);
	let handle = thread::spawn(move || {
		let mut searcher = Finder::new(&search_phrase, location, res_clone);
		searcher.search().unwrap();
	});

	let mut view = View::new(results);

	view.run().unwrap();
	handle.join().unwrap();
}
