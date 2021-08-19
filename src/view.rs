// use cursive::views::{Dialog, TextView};

// enum Event<I> {
// 	Input(I),
// 	Tick,
// }

use crate::model::Content;
use crossterm::{
	cursor::position,
	event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode},
	Result,
};
use std::sync::mpsc;
use std::{io::stdout, thread, time};

const HELP: &str = r#"Blocking poll() & non-blocking read()
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

pub struct View {
	results: Vec<Content>,
	pub rx: mpsc::Receiver<Content>,
}

impl View {
	pub fn new(rx: mpsc::Receiver<Content>) -> Self {
		Self {
			results: Vec::new(),
			rx,
		}
	}
	pub fn run(&mut self) -> Result<()> {
		let mut stdout = stdout();
		self.init(&mut stdout)?;
		self.execute();
		self.exit(&mut stdout)?;
		Ok(())
	}
	fn init(&self, stdout: &mut std::io::Stdout) -> Result<()> {
		println!("{}", HELP);
		enable_raw_mode()?;
		execute!(stdout, EnableMouseCapture)?;
		Ok(())
	}
	fn execute(&mut self) {
		if let Err(e) = self.print_events() {
			eprintln!("Error: {:?}\r", e);
		}
	}
	fn exit(&self, stdout: &mut std::io::Stdout) -> Result<()> {
		execute!(stdout, DisableMouseCapture)?;
		disable_raw_mode()
	}
	fn print_events(&mut self) -> Result<()> {
		loop {
			// Wait up to 1s for another event
			if poll(time::Duration::from_millis(1_000))? {
				// loop {
				// 	let time = time::Duration::from_millis(2000);
				// 	thread::sleep(time);
				// }
				// It's guaranteed that read() wont block if `poll` returns `Ok(true)`
				let event = read()?;

				// println!("Event::{:?}\r", event);

				// if event == Event::Key(KeyCode::Char('c').into()) {
				// 	println!("Cursor position: {:?}\r", position());
				// }

				if event == Event::Key(KeyCode::Esc.into()) {
					break;
				}
			} else {
				// let (x, y) = crossterm::terminal::size().unwrap();
				// println!("{} {}", x, y);
				// // Timeout expired, no event for 1s
				// println!(".\r");
			}
			loop {
				if let Ok(val) = self.rx.try_recv() {
					println!("{}", val);
					&self.results.push(val);
				} else {
					break;
				}
			}
			println!("test");
		}

		Ok(())
	}
}
