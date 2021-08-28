use crate::controller::UserInput;
use crate::model::ModelData;

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
                print!("{}", c);
            }
        }
        println!("{}", user_input);
        content
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
