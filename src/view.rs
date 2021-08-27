// use cursive::views::{Dialog, TextView};

// enum Event<I> {
// 	Input(I),
// 	Tick,
// }

use crate::controller::UserInput;
use crate::model::ModelData;
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
	size: TerminalSize,
	mark: usize,
}

impl View {
	pub fn new() -> Self {
		let (x, y) = crossterm::terminal::size().unwrap();
		Self {
			size: TerminalSize(x, y),
			mark: 0,
		}
	}
	pub fn paint(&self, data: &ModelData, user_input: &UserInput) {
		print!("\x1B[2J");
		let range = self.mark..self.size.1 as usize + self.mark - 2;
		for i in range {
			if let Some(c) = data.results.get(i) {
				print!("{}", c);
			}
		}
		print!("{}\n", user_input);
	}
	pub fn next_page(&mut self) {
		self.mark += self.size.1 as usize;
	}
	pub fn prev_page(&mut self) {
		self.mark -= self.size.1 as usize;
	}
	pub fn handle_resize(&mut self, x: u16, y: u16) {
		self.size = TerminalSize::new(x, y);
	}
}
