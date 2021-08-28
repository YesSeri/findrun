mod args_parser;
use args_parser::get_args;
mod model;
use model::{Content, Finder, ModelData};

// mod quality_control;
// use quality_control::Timer;

mod view;
use std::thread;
use view::View;

mod controller;
use controller::Controller;

use std::sync::mpsc::{channel, Receiver, Sender};
// use std::time::Duration;
fn main() {
	let (search_phrase, location) = get_args();
	let (tx, rx): (Sender<Content>, Receiver<Content>) = channel();
	thread::spawn(move || {
		let finder = Finder::new(&search_phrase, location, tx);
		finder.search().unwrap();
	});

	let model_data = ModelData::new(rx);
	let view = View::new();
	let mut controller = Controller::new(model_data, view);

	controller.run().unwrap();
}
