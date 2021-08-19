mod args_parser;
// use args_parser::get_args;
mod model;
use model::{Content, Finder};

mod quality_control;
use quality_control::Timer;

mod view;
use view::View;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
// use std::time::Duration;
fn main() {
	// let (search_phrase, location) = get_args();
	let (tx, rx): (Sender<Content>, Receiver<Content>) = channel();
	let (search_phrase, location) = (String::from(".pdf"), std::path::PathBuf::from("C:/Games"));
	// dbg!(&search_phrase, &location);
	let handle = thread::spawn(move || {
		// let timer = Timer::start_timer();
		let mut timer = Timer::new();
		timer.start();
		let mut searcher = Finder::new(&search_phrase, location);
		searcher.search(tx).unwrap();
		timer.stop();
		println!("{:?}", timer.stop())
	});

	let mut view = View::new(rx);

	view.run().unwrap();
	handle.join().unwrap();
	// for received in rx {
	// 	println!("Got: {:?}", received);
	// }
}
