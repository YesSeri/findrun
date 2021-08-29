use std::fmt;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Content {
	pub path: PathBuf,
	pub file_name: String,
	pub id: usize,
}
impl Content {
	pub fn from(path: PathBuf, file_name: String, id: usize) -> Self {
		Self {
			path,
			file_name,
			id,
		}
	}
	pub fn open_file(&self) {
		match open::that(&self.path) {
			Ok(()) => println!("Opened '{}' successfully.", &self.path.display()),
			Err(err) => {
				panic!(
					"An error occurred when opening '{}': {}",
					&self.path.display(),
					err
				)
			}
		}
	}
}
impl fmt::Display for Content {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"| [{}] | {} {}\r\n",
			&self.id,
			&self.file_name,
			&self.path.parent().unwrap().display()
		)
	}
}
enum SearchStatus {
	Finished(FinishStatus),
	Searching,
}
enum FinishStatus {
	NoResults,
	Results,
}
#[derive(Debug)]
pub struct ModelData {
	pub results: Vec<Content>,
	rx: mpsc::Receiver<Content>,
}
impl ModelData {
	pub fn new(rx: mpsc::Receiver<Content>) -> Self {
		Self {
			results: vec![],
			rx,
		}
	}
	pub fn update_results(&mut self) -> bool {
		let mut has_updated = false;
		while let Ok(val) = self.rx.try_recv() {
			has_updated = true;
			self.results.push(val);
		}
		has_updated
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
	pub fn search(self) -> Result<(), Box<dyn std::error::Error>> {
		let mut id: usize = 0;
		for entry in WalkDir::new(&self.search_location)
		// .filter_entry(|e| !Self::is_ignored(e))
		{
			let entry = match entry {
				Ok(entry) => entry,
				_ => continue,
			};
			let name = entry.file_name().to_str().unwrap().to_string();
			if name.contains(&self.search_phrase) {
				let path = entry.path().to_owned();
				let content = Content::from(path, name, id);
				id += 1;
				self.tx.send(content).unwrap();
			}
		}
		if id == 0 {
			return Err("No results were found".into());
		}
		println!("Search finished.");
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {}
}
