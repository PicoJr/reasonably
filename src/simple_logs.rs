#![allow(non_snake_case)]
use std::collections::{HashSet, VecDeque};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Readable, Signal};
use dioxus::prelude::*;

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

#[component]
pub(crate) fn Logs(
    researched: Signal<HashSet<String>>,
    logs: Signal<SimpleLogs>
) -> Element {
    rsx! {
        if researched().contains("logs") {
            div {
                class: "logs",
                {logs.read().render()}
            }
        }
    }
}
