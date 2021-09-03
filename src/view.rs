use crate::controller::UserInput;
use crate::model::{Content, ModelData};
use std::fmt;
use std::slice::Iter;

struct TerminalSize(u16, u16);
impl TerminalSize {
	fn new(x: u16, y: u16) -> Self {
		Self(x, y)
	}
}
pub struct View {
	term_size: TerminalSize,
	entry_mark: usize,
	page_mark: usize,
}
impl fmt::Display for Content {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}] | {}", self.id, self.file_name)
	}
}

impl View {
	pub fn new() -> Self {
		let (x, y) = crossterm::terminal::size().unwrap();
		Self {
			term_size: TerminalSize(x, y),
			entry_mark: 0,
			page_mark: 0,
		}
	}
	pub fn paint(&self, data: &ModelData, user_input: &UserInput)
	//  -> Option<crate::model::Content>
	{
		print!("\x1B[2J");
		let (start, end) = self.result_display_interval();

		for i in start..=end {
			let content = data.results.get(i);
			if let Some(c) = content {
				let s = self.format_content(c);
				println!("{}", s);
			} else {
				break;
			}
		}
		print!("Status - ");
		if let Some(outcome) = &data.outcome {
			print!("{}", outcome);
		} else {
			print!("searching");
		}
		print!(" | Input:  {}\r\n", user_input);
	}

	fn format_content(&self, c: &Content) -> String {
		let max_width = self.term_size.0 as usize;
		let mut path = c.path.to_string_lossy().to_string();
		let mut s = format!("| [{}] | {} | {}", c.id, c.file_name, &path);
		let width_left = max_width - 7 + c.id.to_string().len();
		if s.len() > width_left {
			let mut diff = s.len() - width_left;
			while diff != 0 {
				path.pop();
				diff -= 1;
			}
			// Three pops so there is room for three dots at the end to indicate the path has been chopped off.
			path.pop();
			path.pop();
			path.pop();
			s = format!("| [{}] | {} | {}...", c.id, c.file_name, path);
		}
		s
	}
	fn per_page(&self) -> usize {
		let height: usize = self.term_size.1.into();
		height - 3
	}
	fn result_display_interval(&self) -> (usize, usize) {
		let start = self.page_mark * self.per_page();
		let end = start + self.per_page() - 1;
		(start, end)
	}
	pub fn next_page(&mut self, len: usize) {
		let (_, end) = self.result_display_interval();
		if end + 1 < len {
			self.page_mark += 1;
		}
	}
	pub fn prev_page(&mut self) {
		self.page_mark = self.page_mark.saturating_sub(1);
	}
	pub fn next_entry(&mut self) {}
	pub fn prev_entry(&mut self) {}
	pub fn handle_resize(&mut self, x: u16, y: u16) {
		// If the marker is out of bound on resize, we move the marker.
		if ((y - 3) as usize) < self.entry_mark {
			self.entry_mark = (y - 3) as usize
		}
		self.term_size = TerminalSize::new(x, y);
	}
}
