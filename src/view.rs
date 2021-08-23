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
use std::sync::{Arc, Mutex, MutexGuard};
use std::{io::stdout, thread, time};

const HELP: &str = r#"Blocking poll() & non-blocking read()
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;
struct Ui {
	selected: Option<u32>,
	input: String,
}
impl Ui {
	fn new() -> Self {
		Self {
			selected: None,
			input: "".to_string(),
		}
	}
	fn do_event(&self, key: char) {}
	fn paint(&self, res: &Vec<Content>) {
		for el in res {
			println!("{:?}", el);
		}
		println!("{:?}", self.input);
		println!("");
	}
	fn backspace(&mut self) {
		let mut chars = self.input.chars();
		chars.next_back();
		self.input = chars.as_str().to_string();
	}
	fn add_num(&mut self, c: char) {
		if c.is_numeric() {
			self.input = format!("{}{}", self.input, c);
		}
	}
}

pub struct View {
	results: Arc<Mutex<Vec<Content>>>,
	prev_len: usize,
	ui: Ui,
}

impl View {
	pub fn new(results: Arc<Mutex<Vec<Content>>>) -> Self {
		Self {
			results,
			prev_len: 0,
			ui: Ui::new(),
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
				// It's guaranteed that read() wont block if `poll` returns `Ok(true)`
				let event = read()?;
				let key = match event {
					Event::Key(key) => Some(key.code),
					_ => None,
				};
				match key.unwrap() {
					KeyCode::Char(c) => {
						self.ui.add_num(c);
					}
					KeyCode::Backspace => {
						self.ui.backspace();
					}
					KeyCode::Esc => break,
					_ => (),
				};
			// self.ui.paint();

			// if event == Event::Key(KeyCode::Char('c').into()) {
			// 	println!("Cursor position: {:?}\r", position());
			// }
			// if event == Event::Key(KeyCode::Esc.into()) {
			// panic!();
			// break;
			// }
			} else {
				// let (x, y) = crossterm::terminal::size().unwrap();
				// println!("{} {}", x, y);
				// // Timeout expired, no event for 1s
				// println!(".\r");
			}
			let res = &*self.results.lock().unwrap();
			let len = res.len();
			println!("new loop");

			if self.prev_len != len {
				self.prev_len = len;
				self.ui.paint(res);
			}
		}
		Ok(())
	}
}
