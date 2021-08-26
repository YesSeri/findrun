mod args_parser;
// use args_parser::get_args;
mod model;
use model::{Content, Finder, ModelData};

// mod quality_control;
// use quality_control::Timer;

mod view;
use view::View;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
// use std::time::Duration;
fn main() {
	// let (search_phrase, location) = get_args();
	let (tx, rx): (Sender<Content>, Receiver<Content>) = channel();
	let (search_phrase, location) = (
		String::from(".pdf"),
		// std::path::PathBuf::from("/home/henrikz/Downloads"),
		std::path::PathBuf::from("C:/Games"),
	);
	let handle = thread::spawn(move || {
		let finder = Finder::new(&search_phrase, location, tx);
		finder.search().unwrap();
	});

	let model_data = ModelData::new(rx);
	let mut view = View::new(model_data);

	view.run().unwrap();

	handle.join().unwrap();
}
