use std::collections::VecDeque;

pub(crate) struct SimpleLogs {
    max_lines: usize,
    lines: VecDeque<String>,
}

impl SimpleLogs {
    pub(crate) fn new() -> Self {
        SimpleLogs {
            max_lines: 5,
            lines: VecDeque::new(),
        }
    }

    pub(crate) fn render(&self) -> String {
        let lines: Vec<String> = self.lines.iter().cloned().collect();
        lines.join("\n")
    }

    pub(crate) fn log(&mut self, message: &str) {
        if self.lines.len() >= self.max_lines {
            self.lines.pop_front();
        }
        self.lines.push_back(message.to_string());
    }
}
