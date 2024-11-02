#![allow(non_snake_case)]

mod simple_logs;

use simple_logs::SimpleLogs;

use std::collections::{HashSet, VecDeque};

use break_infinity::{sum_geometric_series, Decimal};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use async_std::task::sleep;

#[derive(Clone, Debug, PartialEq)]
enum Theme {
    LightTheme,
    DarkTheme,
}

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
            p {"{description}"}
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
fn ToggleThemeAction(
    mut logs: Signal<SimpleLogs>,
    mut theme: Signal<Theme>,
) -> Element {
    let current_theme: Theme = theme();
    rsx! {
        button { onclick: move |_| {
            match current_theme {
                Theme::LightTheme => {
                    *theme.write() = Theme::DarkTheme;
                    logs.write().log(
                        "toggling theme...now dark"
                    );
                    spawn(async move {
                        eval(r#"
                        document.documentElement.setAttribute('data-theme', "dark")
                        "#,
                        ).await.expect("failed to run JS");
                    });
                },
                Theme::DarkTheme => {
                    *theme.write() = Theme::LightTheme;
                    logs.write().log(
                        "toggling theme...now light"
                    );
                    spawn(async move {
                        eval(r#"
                        document.documentElement.setAttribute('data-theme', "light")
                        "#,
                        ).await.expect("failed to run JS");
                    });
                },
            };
        }
        , "Toggle Theme" }
    }
}

#[component]
fn ResearchOnce(
    mut logs: Signal<SimpleLogs>,
    mut researched: Signal<HashSet<String>>,
    mut loc: Signal<Decimal>,
    research_name: String,
    button_name: String,
    debug_message: String,
    description: String,
    loc_cost: Decimal,
) -> Element {
    let disabled = loc() < loc_cost;
    rsx! {
        div {
            class: "research",
            p {"{description}"}
            p {"Cost {loc_cost} loc"}
            button {
                class: "research-button",
                disabled: disabled,
                onclick: move |_| {
                researched.write().insert(research_name.clone()) ;
                logs.write().log(
                    &debug_message
                );
                loc -= loc_cost;
            }
            , {button_name} }
        }
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
    let research_internship_loc_cost = Decimal::new(10.0);
    let research_code_metrics_loc_cost = Decimal::new(10.0);
    let research_rmrf_loc_cost = Decimal::new(10.0);

    let logs: Signal<SimpleLogs> = use_signal(SimpleLogs::new);
    let researched: Signal<HashSet<String>> = use_signal(HashSet::new);
    let theme: Signal<Theme> = use_signal(|| Theme::LightTheme);

    // stats
    let mut loc_dt: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let mut bugs_dt: Signal<Decimal> = use_signal(|| Decimal::ZERO);

    // clicks
    let mut code_clicks: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let mut debug_clicks: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let mut interns_clicks: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let mut rmrf_clicks: Signal<Decimal> = use_signal(|| Decimal::ZERO);

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

    // placeholder
    let placeholder: Signal<Decimal> = use_signal(|| Decimal::ZERO);

    use_future(move || async move {
        let dt_milliseconds = 100;
        loop {
            let dt = Decimal::new(0.01);
            // loc produced by clicking on the code button
            let manual_loc = code_clicks() * loc_per_clicks();
            // interns hired by clicking the hire interns button
            let manual_interns = interns_clicks();
            // bugs produced as a byproduct of clicking the code button
            // subtracting bugs removed by clicking the debug button
            let manual_bugs =
                manual_loc * manual_bugs_ratio() - debug_clicks() * debug_per_clicks();

            // purchases
            // must be computed before incrementing interns
            let manual_interns_loc_cost = sum_geometric_series(
                &manual_interns,
                &interns_loc_base_cost,
                &interns_loc_growth_rate,
                &interns(),
            );

            // loc produced by interns, ...
            let auto_loc = interns() * interns_loc_dt() * dt;
            // bugs produced by interns, ...
            let auto_bugs = interns() * interns_loc_dt() * interns_bugs_ratio() * dt;

            // update loc, accounting all sources
            loc += manual_loc - manual_interns_loc_cost + auto_loc;
            // update live code metrics
            *loc_dt.write() =
                (manual_loc + auto_loc) * Decimal::new(1e3 / (dt_milliseconds as f64));

            // update bugs, accouting for all sources
            bugs += manual_bugs + auto_bugs;
            // update live code metrics
            *bugs_dt.write() =
                (manual_bugs + auto_bugs) * Decimal::new(1e3 / (dt_milliseconds as f64));

            // update interns count, accouting for all sources
            interns += manual_interns;

            // handle rm -rf
            if rmrf_clicks() > Decimal::ZERO {
                *loc.write() = Decimal::ZERO;
                *bugs.write() = Decimal::ZERO;
            }

            // reset clicks, now that all clicks have been taken into account
            *code_clicks.write() = Decimal::ZERO;
            *debug_clicks.write() = Decimal::ZERO;
            *interns_clicks.write() = Decimal::ZERO;
            *rmrf_clicks.write() = Decimal::ZERO;

            // sleep before next tick
            sleep(std::time::Duration::from_millis(dt_milliseconds)).await;
        }
    });

    rsx! {
        div { // vertical
            class: "everything",
            Logs {logs}
            div { // vertical
                class: "metrics",
                if researched().contains("code_metrics") {
                    p {"LOC/s {loc_dt().floor()}"}
                    p {"bugs/s {bugs_dt().floor()}"}
                }
                if interns() > Decimal::ZERO {
                    p {"Interns {interns().floor()}"}
                }
                if loc() > Decimal::ZERO {
                    p {"Lines of code {loc().floor()}"}
                }
                if bugs() > Decimal::ZERO {
                    p {"Bugs {bugs().floor()}"}
                }
            }
            div { // horizontal
                class: "interactions",
                div { // vertical
                    class: "repeatable-actions",
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
                    ToggleThemeAction {
                        logs: logs,
                        theme: theme,
                    }
                    if researched().contains("internship") {
                        RepeatableAction{
                            logs: logs,
                            clicks: interns_clicks,
                            loc: loc,
                            produced: interns,
                            button_name: "hire intern",
                            debug_message: "hiring an intern...",
                            description: "Produces loc, and bugs",
                            loc_base_cost: interns_loc_base_cost,
                            loc_growth_rate: interns_loc_growth_rate,
                        }
                    }
                    if researched().contains("rmrf") {
                        RepeatableAction{
                            logs: logs,
                            clicks: rmrf_clicks,
                            loc: loc,
                            produced: placeholder,
                            button_name: "rm -rf",
                            debug_message: "running rm -rf...",
                            description: "Wipe out all loc and bugs",
                            loc_base_cost: Decimal::ZERO,
                            loc_growth_rate: Decimal::ONE,
                        }
                    }
                }
                div { // vertical
                    class: "researches",
                    if !researched().contains("internship") {
                        ResearchOnce{
                            logs: logs,
                            researched: researched,
                            loc: loc,
                            research_name: "internship",
                            button_name: "research internship",
                            debug_message: "intership researched",
                            description: "Allow hiring interns, who produce loc and bugs automaticaly.",
                            loc_cost: research_internship_loc_cost,
                        }
                    }
                    if !researched().contains("code_metrics") {
                        ResearchOnce{
                            logs: logs,
                            researched: researched,
                            loc: loc,
                            research_name: "code_metrics",
                            button_name: "research code metrics",
                            debug_message: "code metrics researched",
                            description: "Display LOC/s and bugs/s.",
                            loc_cost: research_code_metrics_loc_cost,
                        }
                    }
                    if !researched().contains("rmrf") {
                        ResearchOnce{
                            logs: logs,
                            researched: researched,
                            loc: loc,
                            research_name: "rmrf",
                            button_name: "learn rm -rf",
                            debug_message: "rm -rf researched",
                            description: "For desperate situations, allow using rm-rf command",
                            loc_cost: research_rmrf_loc_cost,
                        }
                    }
                }
            }
        }
    }
}
