#![allow(non_snake_case)]

use std::collections::VecDeque;

use break_infinity::Decimal;
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
fn DebugAction(mut bug_count: Signal<Decimal>) -> Element {
    rsx! {
        button { onclick: move |_| bug_count -= Decimal::new(1.0), "Debug" }
    }
}

#[component]
fn CodeAction(
    mut logs: Signal<SimpleLogs>,
    mut loc_count: Signal<Decimal>,
    mut bug_count: Signal<Decimal>,
) -> Element {
    rsx! {
        button { onclick: move |_| {
            let delta_loc = Decimal::new(1.0);
            let delta_bug = Decimal::new(1.0);
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
    let logs: Signal<SimpleLogs> = use_signal(SimpleLogs::new);
    let loc_count: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let bug_count: Signal<Decimal> = use_signal(|| Decimal::ZERO);

    rsx! {
        Logs {logs}
        div {
            if loc_count() > Decimal::ZERO {
                h1 {"Lines of code {loc_count().floor()}"}
            }
            if bug_count() > Decimal::ZERO {
                h1 {"Bugs {bug_count().floor()}"}
            }
            CodeAction{
                logs,
                loc_count,
                bug_count,
            }
            if bug_count() > Decimal::ZERO {
                DebugAction {bug_count}
            }
        }
    }
}
