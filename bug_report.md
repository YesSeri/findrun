I have found a bug in the interaction between crossterm and the crate open, which is used to open files and urls in their default programs. 

What happens here is that the program waits one second for user actions and if the user presses a key it goes into the if statement and then repeats. This example works and the file gets opened in the foreground as soon as the user presses a key:
```rust
use crossterm::event::poll;
use std::time;

fn main() {
	loop {
		if poll(time::Duration::from_millis(1_000)).unwrap() {
			open::that("C:/Programming/rust/rust-in-action/Rust-In-Action.pdf").unwrap();
		}
	}
}

```
In this example I do the exact same thing, but I also read what key the user has pressed, and if it is `esc` I break the loop, which then quits the program. This example doesn't work as expected and the file gets opened furthest in the background when the user presses a key:
```rust
use crossterm::event::{poll, read, Event, KeyCode};
use std::time;

fn main() {
	loop {
		if poll(time::Duration::from_millis(1_000)).unwrap() {
			open::that("C:/Programming/rust/rust-in-action/Rust-In-Action.pdf").unwrap();
			let event = read().unwrap();
			if event == Event::Key(KeyCode::Esc.into()) {
				break;
			}
		}
	}
}
```

I am quite new to rust, so I don't really have any good ideas on how to debug this further. Do you have an idea? Is it crossterm or is it this crate that has the bug?