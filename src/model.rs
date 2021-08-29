use std::fmt;
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

#[derive(Debug)]
pub struct ModelData {
	pub results: Vec<Content>,
	rx: mpsc::Receiver<Content>,
	rx_outcome: mpsc::Receiver<Option<Outcome>>,
	// Outcome None means we are still searching, and Some means it has finished searching. Either with Outcome::Hits or Outcome::NoHits.
	pub outcome: Option<Outcome>,
}
impl ModelData {
	pub fn new(rx: mpsc::Receiver<Content>, rx_outcome: mpsc::Receiver<Option<Outcome>>) -> Self {
		Self {
			results: vec![],
			rx,
			rx_outcome,
			outcome: None,
		}
	}
	pub fn update_results(&mut self) -> bool {
		let mut has_updated = false;
		while let Ok(val) = self.rx.try_recv() {
			has_updated = true;
			self.results.push(val);
		}
		self.update_outcome();
		has_updated
	}
	fn update_outcome(&mut self) {
		if let Ok(outcome) = self.rx_outcome.try_recv() {
			self.outcome = outcome;
		}
	}
}

#[derive(Debug)]
pub struct Finder<'a> {
	search_phrase: &'a str,
	search_location: PathBuf,
	tx: mpsc::Sender<Content>,
	tx_outcome: mpsc::Sender<Option<Outcome>>,
}
impl<'a> Finder<'a> {
	pub fn new(
		search_phrase: &'a str,
		search_location: PathBuf,
		tx: mpsc::Sender<Content>,
		tx_outcome: mpsc::Sender<Option<Outcome>>,
	) -> Self {
		Self {
			search_phrase,
			search_location,
			tx,
			tx_outcome,
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
	pub fn search(self) {
		let mut id: usize = 0;
		self.tx_outcome.send(None).unwrap();
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
			self.tx_outcome.send(Some(Outcome::NoHits)).unwrap();
		} else {
			self.tx_outcome.send(Some(Outcome::Hits)).unwrap();
		}
	}
}
#[derive(Debug)]
pub enum Outcome {
	NoHits,
	Hits,
}
impl fmt::Display for Outcome {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Outcome::NoHits => {
				write!(f, "Search finished with no results.")
			}
			Outcome::Hits => {
				write!(f, "Search finished: ")
			}
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {}
}
