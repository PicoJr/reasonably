#![allow(non_snake_case)]

use std::collections::{HashSet, VecDeque};

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
fn RepeatableAction(
    mut logs: Signal<SimpleLogs>,
    mut clicks: Signal<Decimal>,
    loc: Signal<Decimal>,
    produced: Signal<Decimal>,
    button_name: String,
    debug_message: String,
    description: String,
    loc_base_cost: Decimal,
    loc_growth_rate: Decimal,
) -> Element {
    let new_instances = produced() + clicks();
    let loc_cost = loc_base_cost * loc_growth_rate.pow(&new_instances);
    let disabled = loc() < loc_cost;
    rsx! {
        div {
            class: "repeatable-action",
            p {"You can hire interns who code automaticaly"}
            p {"Cost {loc_cost} loc"}
            button {
                disabled: disabled,
                class: "repeatable-action-button",
                onclick: move |_| {
                clicks += Decimal::new(1.0);
                logs.write().log(debug_message.as_str())
            }
            , {button_name} }
        }
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

#[component]
fn ResearchHireIntern(
    mut logs: Signal<SimpleLogs>,
    mut researched: Signal<HashSet<String>>,
    loc: Signal<Decimal>,
) -> Element {
    let loc_cost = Decimal::new(1e1);
    let disabled = loc() < loc_cost;
    rsx! {
        div {
            class: "research",
            p {"You can hire interns who code automaticaly"}
            p {"Cost {loc_cost} loc"}
            button {
                class: "research-button",
                disabled: disabled,
                onclick: move |_| {
                researched.write().insert("internship".to_string()) ;
                logs.write().log(
                    "internship researched..."
                );
                loc -= loc_cost;
            }
            , "research internship" }
        }
    }
}

#[component]
fn ResearchCodeMetrics(
    mut logs: Signal<SimpleLogs>,
    mut researched: Signal<HashSet<String>>,
    loc: Signal<Decimal>,
) -> Element {
    let loc_cost = Decimal::new(1e1);
    let disabled = loc() < loc_cost;
    rsx! {
        div {
            class: "research",
            p {"You can monitor the loc/s and bugs/s"}
            p {"Cost {loc_cost} loc"}
            button {
                class: "research-button",
                disabled: disabled,
                onclick: move |_| {
                researched.write().insert("code_metrics".to_string()) ;
                logs.write().log(
                    "code metrics researched..."
                );
                loc -= loc_cost;
            }
            , "research code metrics" }
        }
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
    let interns_loc_base_cost = Decimal::new(10.0);
    let interns_loc_growth_rate = Decimal::new(1.1);

    let logs: Signal<SimpleLogs> = use_signal(SimpleLogs::new);
    let researched: Signal<HashSet<String>> = use_signal(HashSet::new);

    // stats
    let mut loc_dt: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let mut bugs_dt: Signal<Decimal> = use_signal(|| Decimal::ZERO);

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
        let dt_milliseconds = 100;
        loop {
            let dt = Decimal::new(0.01);
            let manual_loc = code_clicks() * loc_per_clicks();
            let manual_interns = interns_clicks();
            let manual_bugs =
                manual_loc * manual_bugs_ratio() - debug_clicks() * debug_per_clicks();

            let auto_loc = interns() * interns_loc_dt() * dt;
            let auto_bugs = interns() * interns_loc_dt() * interns_bugs_ratio() * dt;

            loc += manual_loc + auto_loc;
            *loc_dt.write() =
                (manual_loc + auto_loc) * Decimal::new(1e3 / (dt_milliseconds as f64));
            bugs += manual_bugs + auto_bugs;
            *bugs_dt.write() =
                (manual_bugs + auto_bugs) * Decimal::new(1e3 / (dt_milliseconds as f64));
            interns += manual_interns;

            // reset clicks
            *code_clicks.write() = Decimal::ZERO;
            *debug_clicks.write() = Decimal::ZERO;
            *interns_clicks.write() = Decimal::ZERO;
            // sleep before next tick
            sleep(std::time::Duration::from_millis(dt_milliseconds)).await;
        }
    });

    rsx! {
        Logs {logs}
        div {
            if loc() > Decimal::ZERO {
                p {"Lines of code {loc().floor()}"}
            }
            if researched().contains("code_metrics") {
                p {"LOC/s {loc_dt().floor()}"}
            }
            if bugs() > Decimal::ZERO {
                p {"Bugs {bugs().floor()}"}
            }
            if researched().contains("code_metrics") {
                p {"bugs/s {bugs_dt().floor()}"}
            }
            if interns() > Decimal::ZERO {
                p {"Interns {interns().floor()}"}
            }
            CodeAction{
                logs,
                code_clicks,
            }
            if bugs() > Decimal::ZERO {
                DebugAction {
                    logs,
                    debug_clicks,
                }
            }
            if !researched().contains("internship") {
                ResearchHireIntern {
                    logs,
                    researched,
                    loc,
                }
            }
            if !researched().contains("code_metrics") {
                ResearchCodeMetrics {
                    logs,
                    researched,
                    loc,
                }
            }
            if researched().contains("internship") {
                HireInternAction {
                    logs,
                    interns_clicks,
                }
            }
            RepeatableAction{
                logs: logs,
                clicks: interns_clicks,
                loc: loc,
                produced: interns,
                button_name: "hire intern",
                debug_message: "debug message",
                description: "description",
                loc_base_cost: interns_loc_base_cost,
                loc_growth_rate: interns_loc_growth_rate,
            }
        }
    }
}
