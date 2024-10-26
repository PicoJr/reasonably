#![allow(non_snake_case)]

use std::collections::VecDeque;
use std::fmt;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn DebugAction(mut bug_count: Signal<i32>) -> Element {
    rsx! {
        button { onclick: move |_| bug_count -= 1, "Debug" }
    }
}

#[component]
fn CodeAction(
    mut logs: Signal<SimpleLogs>,
    mut loc_count: Signal<i32>,
    mut bug_count: Signal<i32>,
    loc_increment: Signal<i32>,
    bug_loc_ratio: Signal<f32>,
) -> Element {
    rsx! {
        button { onclick: move |_| {
            let delta_loc = loc_increment();
            let delta_bug = (loc_increment() as f32 * bug_loc_ratio()) as i32;
            loc_count += delta_loc;
            bug_count += delta_bug;
            logs.write().log(
                format!("coding...+{} loc +{} bugs", delta_loc, delta_bug).as_str()
            )
        }
        , "Code" }
    }
}

struct SimpleLogs {
    max_lines: usize,
    lines: VecDeque<String>,
}

impl SimpleLogs {
    fn new() -> Self {
        SimpleLogs {
            max_lines: 5,
            lines: VecDeque::new(),
        }
    }

    fn render(&self) -> String {
        let lines: Vec<String> = self.lines.iter().cloned().collect();
        lines.join("\n")
    }

    fn log(&mut self, message: &str) {
        if self.lines.len() >= self.max_lines {
            self.lines.pop_front();
        }
        self.lines.push_back(message.to_string());
    }
}

#[component]
fn Logs(logs: Signal<SimpleLogs>) -> Element {
    rsx! {
        div {
            class: "logs",
            {logs.read().render()}
        }
    }
}

#[component]
fn Home() -> Element {
    let mut logs: Signal<SimpleLogs> = use_signal(SimpleLogs::new);
    let mut loc_count: Signal<i32> = use_signal(|| 0);
    let mut loc_increment: Signal<i32> = use_signal(|| 1);
    let bug_count: Signal<i32> = use_signal(|| 0);
    let bug_loc_ratio: Signal<f32> = use_signal(|| 2.0);

    rsx! {
        Logs {logs}
        div {
            if loc_count() > 0 {
                h1 {"Lines of code {loc_count}"}
            }
            if bug_count() > 0 {
                h1 {"Bugs {bug_count}"}
            }
            CodeAction{
                logs,
                loc_count,
                bug_count,
                loc_increment,
                bug_loc_ratio,
            }
            if bug_count() > 0 {
                DebugAction {bug_count}
            }
            if loc_count() >= 20 && bug_count() <= 0 {
                button {onclick: move |_| {
                    loc_count -= 20;
                }, "Write a hello world"}
            }

            if loc_count() >= 10 && bug_count() <= 0 {
                button {onclick: move |_| {
                    loc_count -= 10;
                    loc_increment += 1;
                }, "Learn VIM"}
            }
        }
    }
}
