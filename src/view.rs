// use cursive::views::{Dialog, TextView};

// enum Event<I> {
// 	Input(I),
// 	Tick,
// }

use crate::model::ModelData;
use crossterm::{
	// cursor::position,
	event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode},
	Result,
};
use std::{fmt, io::stdout, time};

// const HELP: &str = r#"Blocking poll() & non-blocking read()
//  - Keyboard, mouse and terminal resize events enabled
//  - Prints "." every second if there's no event
//  - Hit "c" to print current cursor position
//  - Use Esc to quit
// "#;

struct TerminalSize(u16, u16);
impl TerminalSize {
	fn new(x: u16, y: u16) -> Self {
		Self(x, y)
	}
}
pub struct View {
	model_data: ModelData,
	size: TerminalSize,
	mark: usize,
}

impl View {
	pub fn new(model_data: ModelData) -> Self {
		let (x, y) = crossterm::terminal::size().unwrap();
		Self {
			model_data,
			size: TerminalSize(x, y),
			mark: 0,
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
		// println!("{}", HELP);
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
	fn paint(&self) {
		println!("{}", self);
	}
	fn handle_resize(&mut self, x: u16, y: u16) {
		self.size = TerminalSize::new(x, y);
	}
	fn print_events(&mut self) -> Result<()> {
		loop {
			let has_updated = self.model_data.update_results();
			if has_updated {
				self.paint();
			}
			// Wait up to 1s for another event
			if poll(time::Duration::from_millis(1_000))? {
				// loop {
				// 	let time = time::Duration::from_millis(2000);
				// 	thread::sleep(time);
				// }
				// It's guaranteed that read() wont block if `poll` returns `Ok(true)`
				let event = read()?;
				match event {
					Event::Key(k) => self.model_data.input(k.code),
					Event::Mouse(m) => println!("Mouse event: {:?}", m),
					Event::Resize(x, y) => self.handle_resize(x, y),
				}

				// println!("Event::{:?}\r", event);
				if event == Event::Key(KeyCode::Esc.into()) {
					break;
				}
				self.paint();
			} else {
				// let (x, y) = crossterm::terminal::size().unwrap();
				// println!("{} {}", x, y);
				// // Timeout expired, no event for 1s
				// println!(".\r");
			}
		}

		Ok(())
	}
}
impl fmt::Display for View {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut i: usize = 0;
		loop {
			writeln!(f, "{}", &self.model_data.results.get(i).unwrap());
			i += 1;
			if i == self.size.1.into() || i == self.model_data.results.len() {
				break;
			}
		}
		// for r in self.model_data.results.iter() {
		// 	writeln!(f, "{}", r);
		// }
		write!(f, "{}", self.model_data.get_input())
	}
}
