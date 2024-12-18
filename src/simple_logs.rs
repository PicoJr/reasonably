#![allow(non_snake_case)]
use crate::constants::Research;
use crate::state::State;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus::prelude::{Readable, Signal};
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

#[component]
pub(crate) fn Logs(state: Signal<State>) -> Element {
    rsx! {
        if state.read().researched.contains(&Research::Logs) {
            div {
                class: "logs",
                {state.read().logs.render()}
            }
        }
    }
}
