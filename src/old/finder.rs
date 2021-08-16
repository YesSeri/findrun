use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::io::Error;
use std::path;
#[derive(Debug, Clone)]
pub struct File {
	pub is_dir: bool,
	pub name: String,
	pub path: path::PathBuf,
}
impl Display for File {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		writeln!(f, "{} - {}", self.name, self.path.to_string_lossy())
	}
}
impl File {
	pub fn from(is_dir: bool, name: String, path: path::PathBuf) -> Self {
		File { is_dir, name, path }
	}
	pub fn get_content(&self) -> Option<Vec<File>> {
		if self.is_dir {
			let mut vec: Vec<File> = Vec::new();
			if let Ok(entries) = fs::read_dir(&self.path) {
				for entry in entries {
					if let Ok(entry) = entry {
						let path = &entry.path();
						let file_name = entry.file_name().into_string().unwrap();
						let is_dir = path.metadata().unwrap().is_dir();
						let file = File::from(is_dir, file_name, entry.path());
						vec.push(file);
					}
				}
			}
			Some(vec)
		} else {
			None
		}
	}
	pub fn open_file(&self) -> Result<(), Error> {
		open::that(&self.path)
	}
}

pub struct View {
	results: Vec<File>,
}
impl View {
	pub fn from(results: Vec<File>) -> Self {
		Self { results }
	}
}
impl Display for View {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		if self.results.len() == 0 {
			write!(f, "No Results");
		} else {
			for (i, res) in self.results.iter().enumerate() {
				write!(f, "[{}] - {}", i, res);
			}
		}
		Ok(())
	}
}
