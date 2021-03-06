mod args_parser;
use args_parser::get_args;
mod model;
use model::{Content, Finder, ModelData, Outcome};

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
	let (tx_outcome, rx_outcome): (Sender<Option<Outcome>>, Receiver<Option<Outcome>>) = channel();
	thread::spawn(move || {
		let finder = Finder::new(&search_phrase, location, tx, tx_outcome);
		finder.search();
	});

	let model_data = ModelData::new(rx, rx_outcome);
	let view = View::new();
	let mut controller = Controller::new(model_data, view);

	controller.run().unwrap();
}
