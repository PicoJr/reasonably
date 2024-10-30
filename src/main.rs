#![allow(non_snake_case)]

use std::collections::VecDeque;

use break_infinity::Decimal;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use async_std::task::sleep;

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
fn DebugAction(mut logs: Signal<SimpleLogs>, mut debug_clicks: Signal<Decimal>) -> Element {
    rsx! {
        button { onclick: move |_| {
            debug_clicks += Decimal::new(1.0);
            logs.write().log(
                "debugging..."
            )
        }
        , "Debug" }
    }
}

#[component]
fn CodeAction(mut logs: Signal<SimpleLogs>, mut code_clicks: Signal<Decimal>) -> Element {
    rsx! {
        button { onclick: move |_| {
            code_clicks += Decimal::new(1.0);
            logs.write().log(
                "coding..."
            )
        }
        , "Code" }
    }
}

#[component]
fn HireInternAction(mut logs: Signal<SimpleLogs>, mut interns_clicks: Signal<Decimal>) -> Element {
    rsx! {
        button { onclick: move |_| {
            interns_clicks += Decimal::new(1.0);
            logs.write().log(
                "hiring an intern..."
            )
        }
        , "Hire interns" }
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

    // clicks
    let mut code_clicks: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let mut debug_clicks: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let mut interns_clicks: Signal<Decimal> = use_signal(|| Decimal::ZERO);

    // production per clicks
    let loc_per_clicks: Signal<Decimal> = use_signal(|| Decimal::new(1.0));
    let debug_per_clicks: Signal<Decimal> = use_signal(|| Decimal::new(1.0));

    let manual_bugs_ratio: Signal<Decimal> = use_signal(|| Decimal::new(1.0));
    let interns_bugs_ratio: Signal<Decimal> = use_signal(|| Decimal::new(2.0));

    // resources
    let mut loc: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let mut bugs: Signal<Decimal> = use_signal(|| Decimal::ZERO);

    // producers
    let mut interns: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let interns_loc_dt: Signal<Decimal> = use_signal(|| Decimal::new(1.0));

    use_future(move || async move {
        loop {
            let dt = Decimal::new(0.01);
            let manual_loc = code_clicks() * loc_per_clicks();
            let manual_interns = interns_clicks();
            let manual_bugs =
                manual_loc * manual_bugs_ratio() - debug_clicks() * debug_per_clicks();

            let auto_loc = interns() * interns_loc_dt() * dt;
            let auto_bugs = interns() * interns_bugs_ratio() * dt;

            loc += manual_loc + auto_loc;
            bugs += manual_bugs + auto_bugs;
            interns += manual_interns;

            // reset clicks
            *code_clicks.write() = Decimal::ZERO;
            *debug_clicks.write() = Decimal::ZERO;
            *interns_clicks.write() = Decimal::ZERO;
            // sleep before next tick
            sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    rsx! {
        Logs {logs}
        div {
            if loc() > Decimal::ZERO {
                h1 {"Lines of code {loc().floor()}"}
            }
            if bugs() > Decimal::ZERO {
                h1 {"Bugs {bugs().floor()}"}
            }
            if interns() > Decimal::ZERO {
                h1 {"Interns {interns().floor()}"}
            }
            CodeAction{
                logs,
                code_clicks,
            }
            HireInternAction {
                logs,
                interns_clicks,
            }
            if bugs() > Decimal::ZERO {
                DebugAction {
                    logs,
                    debug_clicks,
                }
            }
        }
    }
}
