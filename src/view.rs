use crate::controller::UserInput;
use crate::model::{Content, ModelData};

struct TerminalSize(u16, u16);
impl TerminalSize {
	fn new(x: u16, y: u16) -> Self {
		Self(x, y)
	}
}
pub struct View {
	size: TerminalSize,
	entry_mark: usize,
	page_mark: usize,
	// search_status: bool,
}

impl View {
	pub fn new() -> Self {
		let (x, y) = crossterm::terminal::size().unwrap();
		Self {
			size: TerminalSize(x, y),
			entry_mark: 0,
			page_mark: 0,
		}
	}
	pub fn paint(&self, data: &ModelData, user_input: &UserInput) -> Option<crate::model::Content> {
		print!("\x1B[2J");
		let mut content = None;
		for (entry_idx, i) in
			(self.page_mark..self.size.1 as usize + self.page_mark - 2).enumerate()
		{
			if let Some(c) = data.results.get(i) {
				if entry_idx == self.entry_mark {
					print!(">>>");
					content = Some(c.clone());
				}
				self.print_content(c);
			}
		}
		if let Some(outcome) = &data.outcome{
			print!("{}", outcome)
		}else{
			print!("Nothing", )

		}

		println!("{}", user_input);
		content
	}
	fn print_content(&self, c: &Content) {
		let max_width = self.size.0;
		let mut path = c
			.path
			.parent()
			.unwrap()
			.to_string_lossy()
			.to_owned()
			.to_string();
		let s = format!("| [{}] | {} {}", c.id, c.file_name, path);
		if s.len() > max_width as usize {
			let mut diff = s.len() - max_width as usize;
			while diff != 0 {
				path.pop();
				diff -= 1;
			}
			// Three pops so there is room for three dots at the end to indicate the path has been chopped off.
			path.pop();
			path.pop();
			path.pop();
			let s = format!("| [{}] | {} {}...", c.id, c.file_name, path);
			print!("{}\r\n", s);
		} else {
			print!("{}\r\n", s);
		}
		// print!(
		// 	"| [{}] | {} {}\r\n",
		// 	c.id,
		// 	c.file_name,
		// 	c.path.parent().unwrap().display()
		// );
	}
	pub fn next_page(&mut self, len: usize) {
		if self.page_mark > len.saturating_sub(self.size.1 as usize) {
			return;
		}
		self.page_mark += self.size.1 as usize;
	}
	pub fn prev_page(&mut self) {
		self.page_mark = self.page_mark.saturating_sub(self.size.1 as usize);
	}
	pub fn next_entry(&mut self) {
		if self.entry_mark > self.size.1 as usize - 4 {
			return;
		}
		self.entry_mark += 1;
	}
	pub fn prev_entry(&mut self) {
		self.entry_mark = self.entry_mark.saturating_sub(1);
	}
	pub fn handle_resize(&mut self, x: u16, y: u16) {
		if ((y - 3) as usize) < self.entry_mark {
			self.entry_mark = (y - 3) as usize
		}
		self.size = TerminalSize::new(x, y);
	}
}
