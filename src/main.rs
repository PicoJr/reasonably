#![allow(non_snake_case)]

mod simple_logs;
mod repeatable_action;
mod toggle_theme_action;
mod research_once;
mod code_action;
mod debug_action;
mod format_decimal;

use simple_logs::{SimpleLogs, Logs};
use repeatable_action::RepeatableAction;
use toggle_theme_action::ToggleThemeAction;
use research_once::ResearchOnce;
use format_decimal::{format_decimal_devs, format_decimal_bugs, format_decimal_loc};

use std::collections::{HashSet};

use break_infinity::{sum_geometric_series, Decimal};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use async_std::task::sleep;
use crate::code_action::CodeAction;
use crate::debug_action::DebugAction;

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
fn Home() -> Element {
    // repeatable actions costs
    let interns_loc_base_cost = Decimal::new(10.0);
    let interns_loc_growth_rate = Decimal::new(1.1);
    let junior_devs_loc_base_cost = Decimal::new(20.0);
    let junior_devs_loc_growth_rate = Decimal::new(1.1);
    let senior_devs_loc_base_cost = Decimal::new(40.0);
    let senior_devs_loc_growth_rate = Decimal::new(1.1);

    // research costs
    let research_internship_loc_cost = Decimal::new(1.0);
    let research_interns_promotion_loc_cost = Decimal::new(1.0);
    let research_junior_devs_promotion_loc_cost = Decimal::new(1.0);
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
    let mut junior_devs_clicks: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let mut senior_devs_clicks: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let mut rmrf_clicks: Signal<Decimal> = use_signal(|| Decimal::ZERO);

    // production per clicks
    let loc_per_clicks: Signal<Decimal> = use_signal(|| Decimal::new(1.0));
    let debug_per_clicks: Signal<Decimal> = use_signal(|| Decimal::new(1.0));

    let manual_bugs_ratio: Signal<Decimal> = use_signal(|| Decimal::new(1.0));
    let interns_bugs_ratio: Signal<Decimal> = use_signal(|| Decimal::new(2.0));
    let junior_devs_bugs_ratio: Signal<Decimal> = use_signal(|| Decimal::new(1.5));
    let senior_devs_bugs_ratio: Signal<Decimal> = use_signal(|| Decimal::new(1.0));

    // resources
    let mut loc: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let mut bugs: Signal<Decimal> = use_signal(|| Decimal::ZERO);

    // producers
    let mut interns: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let interns_loc_dt: Signal<Decimal> = use_signal(|| Decimal::new(1.0));
    let mut junior_devs: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let junior_devs_loc_dt: Signal<Decimal> = use_signal(|| Decimal::new(1.5));
    let mut senior_devs: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let senior_devs_loc_dt: Signal<Decimal> = use_signal(|| Decimal::new(2.0));
    let mut retired_devs: Signal<Decimal> = use_signal(|| Decimal::ZERO);

    // placeholder
    let placeholder: Signal<Decimal> = use_signal(|| Decimal::ZERO);

    // promotions & retirement
    // interns -> junior devs
    let interns_promotion_ratio_dt: Signal<Decimal> = use_signal(|| Decimal::new(0.04));
    // junior devs -> senior_devs
    let junior_devs_promotion_ratio_dt: Signal<Decimal> = use_signal(|| Decimal::new(0.02));
    // senior_devs -> retired_devs
    let senior_devs_retirement_ratio_dt: Signal<Decimal> = use_signal(|| Decimal::new(0.01));

    // simulation time between 2 updates
    let mut dt: Signal<Decimal> = use_signal(|| Decimal::new(0.01));

    use_future(move || async move {
        let dt_milliseconds = 100; // real time between 2 updates
        loop {
            // loc produced by clicking on the code button
            let manual_loc = code_clicks() * loc_per_clicks();
            // interns hired by clicking the hire interns button
            let manual_interns = interns_clicks();
            // junior devs hired by clicking the hire junior dev button
            let manual_junior_devs = junior_devs_clicks();
            // senior devs hired by clicking the hire senior dev button
            let manual_senior_devs = senior_devs_clicks();
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
            // must be computed before incrementing junior_devs
            let manual_junior_devs_loc_cost = sum_geometric_series(
                &manual_junior_devs,
                &junior_devs_loc_base_cost,
                &junior_devs_loc_growth_rate,
                &junior_devs(),
            );
            // must be computed before incrementing senior_devs
            let manual_senior_devs_loc_cost = sum_geometric_series(
                &manual_senior_devs,
                &senior_devs_loc_base_cost,
                &senior_devs_loc_growth_rate,
                &senior_devs(),
            );

            // loc produced by interns, ...
            let auto_loc = (
                interns() * interns_loc_dt()
                + junior_devs() * junior_devs_loc_dt()
                + senior_devs() * senior_devs_loc_dt()
            ) * dt();
            // bugs produced by interns, ...
            let auto_bugs = (
                interns() * interns_loc_dt() * interns_bugs_ratio()
                + junior_devs() * junior_devs_loc_dt() * junior_devs_bugs_ratio()
                + senior_devs() * senior_devs_loc_dt() * senior_devs_bugs_ratio()
            ) * dt();

            // update loc, accounting all sources
            loc += manual_loc + auto_loc - (
                manual_interns_loc_cost + manual_junior_devs_loc_cost + manual_senior_devs_loc_cost
            );
            // update live code metrics
            *loc_dt.write() =
                (manual_loc + auto_loc) * Decimal::new(1e3 / (dt_milliseconds as f64));

            // update bugs, accounting for all sources
            bugs += manual_bugs + auto_bugs;
            // update live code metrics
            *bugs_dt.write() =
                (manual_bugs + auto_bugs) * Decimal::new(1e3 / (dt_milliseconds as f64));

            // update interns, junior devs, senior devs count, accounting for all sources
            interns += manual_interns;
            junior_devs += manual_junior_devs;
            senior_devs += manual_senior_devs;

            // handle promotions
            *retired_devs.write() += senior_devs() * senior_devs_retirement_ratio_dt() * dt();
            *senior_devs.write() = senior_devs() * (Decimal::ONE - senior_devs_retirement_ratio_dt() * dt());

            if researched().contains("junior_devs_promotion") {
                *senior_devs.write() += junior_devs() * junior_devs_promotion_ratio_dt() * dt();
                *junior_devs.write() = junior_devs() * (Decimal::ONE - junior_devs_promotion_ratio_dt() * dt());
            }

            if researched().contains("interns_promotion") {
                *junior_devs.write() += interns() * interns_promotion_ratio_dt() * dt();
                *interns.write() = interns() * (Decimal::ONE - interns_promotion_ratio_dt() * dt());
            }

            // handle rm -rf
            if rmrf_clicks() > Decimal::ZERO {
                *loc.write() = Decimal::ZERO;
                *bugs.write() = Decimal::ZERO;
            }

            // reset clicks, now that all clicks have been taken into account
            *code_clicks.write() = Decimal::ZERO;
            *debug_clicks.write() = Decimal::ZERO;
            *interns_clicks.write() = Decimal::ZERO;
            *junior_devs_clicks.write() = Decimal::ZERO;
            *senior_devs_clicks.write() = Decimal::ZERO;
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
                    p {"LOC/s {format_decimal_loc(loc_dt())}"}
                    p {"bugs/s {format_decimal_bugs(bugs_dt())}"}
                    p {"dt {dt()}"}
                }
                if interns() > Decimal::ZERO {
                    p {"Interns {format_decimal_devs(interns())}"}
                }
                if junior_devs() > Decimal::ZERO {
                    p {"Junior devs {format_decimal_devs(junior_devs())}"}
                }
                if senior_devs() > Decimal::ZERO {
                    p {"Senior devs {format_decimal_devs(senior_devs())}"}
                }
                if retired_devs() > Decimal::ZERO {
                    p {"Retired devs {format_decimal_devs(retired_devs())}"}
                }
                if loc() > Decimal::ZERO {
                    p {"Lines of code {format_decimal_loc(loc())}"}
                }
                if bugs() > Decimal::ZERO {
                    p {"Bugs {format_decimal_bugs(bugs())}"}
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
                    button {
                        onclick: move |_| {
                            loc *= Decimal::new(2.0);
                    }
                    , {"cheat loc"} }
                    button {
                        onclick: move |_| {
                            bugs *= Decimal::new(2.0);
                    }
                    , {"cheat bugs"} }
                    button {
                        onclick: move |_| {
                            interns *= Decimal::new(2.0);
                    }
                    , {"cheat interns"} }
                    button {
                        onclick: move |_| {
                            junior_devs *= Decimal::new(2.0);
                    }
                    , {"cheat junior devs"} }
                    button {
                        onclick: move |_| {
                            senior_devs *= Decimal::new(2.0);
                    }
                    , {"cheat senior devs"} }
                    button {
                        onclick: move |_| {
                            dt *= Decimal::new(2.0);
                    }
                    , {"cheat time faster"} }
                    button {
                        onclick: move |_| {
                            dt /= Decimal::new(2.0);
                    }
                    , {"cheat time slower"} }
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
                    if researched().contains("internship") {
                        RepeatableAction{
                            logs: logs,
                            clicks: junior_devs_clicks,
                            loc: loc,
                            produced: junior_devs,
                            button_name: "hire junior dev",
                            debug_message: "hiring a junior dev...",
                            description: "Produces loc, and bugs",
                            loc_base_cost: junior_devs_loc_base_cost,
                            loc_growth_rate: junior_devs_loc_growth_rate,
                        }
                    }
                    if researched().contains("internship") {
                        RepeatableAction{
                            logs: logs,
                            clicks: senior_devs_clicks,
                            loc: loc,
                            produced: senior_devs,
                            button_name: "hire senior dev",
                            debug_message: "hiring a senior dev...",
                            description: "Produces loc, and bugs",
                            loc_base_cost: senior_devs_loc_base_cost,
                            loc_growth_rate: senior_devs_loc_growth_rate,
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
                    if !researched().contains("interns_promotion") {
                        ResearchOnce{
                            logs: logs,
                            researched: researched,
                            loc: loc,
                            research_name: "interns_promotion",
                            button_name: "promote interns",
                            debug_message: "interns promotion researched",
                            description: "Allow interns to be promoted to junior devs",
                            loc_cost: research_interns_promotion_loc_cost,
                        }
                    }
                    if !researched().contains("junior_devs_promotion") {
                        ResearchOnce{
                            logs: logs,
                            researched: researched,
                            loc: loc,
                            research_name: "junior_devs_promotion",
                            button_name: "promote junior devs",
                            debug_message: "junior devs promotion researched",
                            description: "Allow junior devs to be promoted to senior devs",
                            loc_cost: research_junior_devs_promotion_loc_cost,
                        }
                    }
                }
            }
        }
    }
}
