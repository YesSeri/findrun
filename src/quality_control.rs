use std::fmt;
use std::fmt::{Display, Result as FmtResult};
use std::ops::{Add, Sub};
use std::time::{Duration, Instant};

struct SubError {}
impl fmt::Display for SubError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "One or more of the timers are not set correctly")
	}
}

pub struct Timer {
	start: Option<Instant>,
	time: Option<Duration>,
}
impl Timer {
	pub fn new() -> Self {
		Self {
			start: None,
			time: None,
		}
	}
	pub fn start(&mut self) {
		self.start = Some(Instant::now())
	}
	pub fn stop(&mut self) {
		if let Some(start) = self.start {
			self.time = Some(start.elapsed());
		}
	}
}

impl Display for Timer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
		write!(f, "{:?}", self.time.unwrap())
	}
}

// impl Add for Point {

// 	fn add(self, other: Self) -> Duration {
// 		Self {
// 			x: self.x + other.x,
// 			y: self.y + other.y,
// 		}
// 	}
// }

impl Sub for Timer {
	type Output = u128;

	// fn sub(self, rhs: Rhs) -> Self::Output;
	fn sub(self, other: Self) -> Self::Output {
		if let (Some(a), Some(b)) = (self.time, other.time) {
			a.as_millis() - b.as_millis()
		} else {
			0
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
