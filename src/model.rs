use std::fmt::Display;
use std::fmt::Result as FmtResult;
use std::io::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone)]
pub struct Content {
	path: PathBuf,
	file_name: String,
}
impl Content {
	pub fn from(path: PathBuf, file_name: String) -> Self {
		Self { path, file_name }
	}
}
impl Display for Content {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
		write!(f, "{} - {}", &self.file_name, &self.path.display())
	}
}

#[derive(Debug)]
pub struct Finder<'a> {
	search_phrase: &'a str,
	search_location: PathBuf,
	results: Arc<Mutex<Vec<Content>>>,
}
impl<'a> Finder<'a> {
	pub fn new(
		search_phrase: &'a str,
		search_location: PathBuf,
		results: Arc<Mutex<Vec<Content>>>,
	) -> Self {
		Self {
			search_phrase,
			search_location,
			results,
		}
	}

	fn is_ignored(e: &DirEntry) -> bool {
		e.file_name()
			.to_str()
			.map(|s| {
				s.contains("$Recycle.Bin")
				// || s.contains("$WINDOWS")
				// || s.contains("Games")
				// || s.contains("msys64")
				// || s.starts_with('.')
			})
			.unwrap_or(false)
	}
	pub fn search(&mut self) -> Result<(), Error> {
		for entry in WalkDir::new(&self.search_location)
		// .into_iter()
		// .filter_entry(|e| !Self::is_ignored(e))
		{
			let entry = match entry {
				Ok(entry) => entry,
				_ => continue,
			};
			let name = entry.file_name().to_str().unwrap().to_string();
			if name.contains(&self.search_phrase) {
				let path = entry.path().to_owned();
				let content = Content::from(path, name);
				let mut results = self.results.lock().unwrap();
				(*results).push(content);
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
