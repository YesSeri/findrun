use crate::model::ModelData;
use crate::view::View;
use crossterm::{
	// cursor::position,
	event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode},
	Result,
};
use std::{fmt, io::stdout, time};

#[derive(Debug)]
pub enum ControlEvent {
	Update,
	Open,
	Nothing,
}

pub struct Controller {
	data: ModelData,
	view: View,
	user_input: UserInput,
}

impl Controller {
	pub fn new(data: ModelData, view: View) -> Self {
		Self {
			data,
			view,
			user_input: UserInput::new(),
		}
	}
	pub fn run(&mut self) -> Result<()> {
		let mut stdout = stdout();
		self.init(&mut stdout)?;
		self.execute();
		self.exit(&mut stdout)?;
		Ok(())
	}
	fn init(&self, stdout: &mut std::io::Stdout) -> Result<()> {
		// println!("{}", HELP);
		enable_raw_mode()?;
		execute!(stdout, EnableMouseCapture)?;
		Ok(())
	}
	fn execute(&mut self) {
		if let Err(e) = self.print_events() {
			eprintln!("Error: {:?}\r", e);
		}
	}
	fn exit(&self, stdout: &mut std::io::Stdout) -> Result<()> {
		execute!(stdout, DisableMouseCapture)?;
		disable_raw_mode()
	}
	fn print_events(&mut self) -> Result<()> {
		loop {
			let has_updated = self.data.update_results();
			if has_updated {
				self.view.paint(&self.data, &self.user_input);
			}
			if poll(time::Duration::from_millis(1_000))? {
				let event = read()?;
				let control_event: ControlEvent = match event {
					Event::Key(k) => self.handle_keycode(k.code),
					Event::Resize(x, y) => {
						self.view.handle_resize(x, y);
						ControlEvent::Nothing
					}
					Event::Mouse(_) => ControlEvent::Nothing,
				};

				if event == Event::Key(KeyCode::Esc.into()) {
					break;
				}
				match control_event {
					ControlEvent::Open => {
						let i = self.user_input.get_input();
						if let Some(index) = i {
							if let Some(content) = self.data.results.get(index) {
								content.open_file();
							}
						}
					}
					ControlEvent::Update => self.view.paint(&self.data, &self.user_input),
					_ => {}
				}
			}
		}
		Ok(())
	}
	fn handle_keycode(&mut self, keycode: KeyCode) -> ControlEvent {
		match keycode {
			KeyCode::Char(c) => self.user_input.set_char(c),
			KeyCode::Backspace => self.user_input.backspace(),
			KeyCode::Enter => ControlEvent::Open,
			KeyCode::Right => {
				self.view.next_page();
				ControlEvent::Update
			}
			KeyCode::Left => {
				self.view.prev_page();
				ControlEvent::Update
			}
			KeyCode::Esc => {
				println!("Program quit");
				ControlEvent::Open
			}
			_ => ControlEvent::Nothing,
		}
	}
}

#[derive(Debug)]
pub struct UserInput(String);
impl UserInput {
	fn new() -> Self {
		Self("".into())
	}
	fn backspace(&mut self) -> ControlEvent {
		if self.0.is_empty() {
			ControlEvent::Nothing
		} else {
			let mut chars = self.0.chars();
			chars.next_back();
			self.0 = chars.collect();
			ControlEvent::Update
		}
	}

	fn set_char(&mut self, c: char) -> ControlEvent {
		if c.is_ascii_digit() {
			self.0 += &c.to_string();
			ControlEvent::Update
		} else {
			ControlEvent::Nothing
		}
	}
	pub fn get_input(&self) -> Option<usize> {
		self.0.parse::<usize>().ok()
	}
}

impl fmt::Display for UserInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
