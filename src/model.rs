use crossterm::event::KeyCode;
use std::fmt::Display;
use std::fmt::Result as FmtResult;
use std::io::Error;
use std::path::PathBuf;
use std::sync::mpsc;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Content {
	pub path: PathBuf,
	pub file_name: String,
	id: usize,
}
impl Content {
	pub fn from(path: PathBuf, file_name: String, id: usize) -> Self {
		Self {
			path,
			file_name,
			id,
		}
	}
}
impl Display for Content {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
		write!(f, "{} - {}", &self.file_name, &self.path.display())
	}
}

#[derive(Debug)]
pub struct UserInput(String);
impl UserInput {
	fn new() -> Self {
		Self("".into())
	}
	fn input(&mut self, key_code: KeyCode) {
		match key_code {
			KeyCode::Char(c) => self.set_char(c),
			KeyCode::Backspace => self.backspace(),
			_ => {
				println!("other")
			}
		}
	}
	fn backspace(&mut self) {
		let mut chars = self.0.chars();
		chars.next_back();
		self.0 = chars.collect();
	}
	fn set_char(&mut self, c: char) {
		if c.is_ascii_digit() {
			self.0 += &c.to_string();
		}
	}
}

#[derive(Debug)]
pub struct ModelData {
	pub results: Vec<Content>,
	input: UserInput,
	rx: mpsc::Receiver<Content>,
}
impl ModelData {
	pub fn new(rx: mpsc::Receiver<Content>) -> Self {
		Self {
			results: vec![],
			input: UserInput::new(),
			rx,
		}
	}
	pub fn update_results(&mut self) -> bool {
		let mut has_updated = false;
		loop {
			if let Ok(val) = self.rx.try_recv() {
				has_updated = true;
				&self.results.push(val);
			} else {
				break;
			}
		}
		has_updated
	}
	pub fn input(&mut self, key_code: crossterm::event::KeyCode) {
		self.input.input(key_code);
	}
	pub fn get_input(&self) -> &str {
		&self.input.0
	}
}

#[derive(Debug)]
pub struct Finder<'a> {
	search_phrase: &'a str,
	search_location: PathBuf,
	tx: mpsc::Sender<Content>,
}
impl<'a> Finder<'a> {
	pub fn new(
		search_phrase: &'a str,
		search_location: PathBuf,
		tx: mpsc::Sender<Content>,
	) -> Self {
		Self {
			search_phrase,
			search_location,
			tx,
		}
	}

	// fn is_ignored(e: &DirEntry) -> bool {
	// 	e.file_name()
	// 		.to_str()
	// 		.map(|s| {
	// 			s.contains("$Recycle.Bin")
	// 				|| s.contains("$WINDOWS")
	// 				// || s.contains("Games")
	// 				|| s.contains("msys64")
	// 				|| s.starts_with('.')
	// 		})
	// 		.unwrap_or(false)
	// }
	pub fn search(self) -> Result<(), Error> {
		for (i, entry) in WalkDir::new(&self.search_location).into_iter().enumerate()
		// .filter_entry(|e| !Self::is_ignored(e))
		{
			let entry = match entry {
				Ok(entry) => entry,
				_ => continue,
			};
			let name = entry.file_name().to_str().unwrap().to_string();
			if name.contains(&self.search_phrase) {
				let path = entry.path().to_owned();
				let content = Content::from(path, name, i);
				self.tx.send(content).unwrap();
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		// let f = Finder::new("text.txt", std::path::PathBuf::from("./test/bbb"));
		// f.search().unwrap();
	}
}
