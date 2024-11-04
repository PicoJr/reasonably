#![allow(non_snake_case)]

mod simple_logs;
mod repeatable_action;
mod toggle_theme_action;
mod research_once;
mod code_action;
mod debug_action;
mod format_decimal;
mod constants;
mod cheat_action;
mod metrics;
mod resources;

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
use crate::cheat_action::CheatAction;
use crate::code_action::CodeAction;
use crate::constants::GameConstants;
use crate::debug_action::DebugAction;
use crate::metrics::Metrics;
use crate::resources::Resources;

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
    let constants = GameConstants::default();

    let logs: Signal<SimpleLogs> = use_signal(SimpleLogs::new);
    let researched: Signal<HashSet<String>> = use_signal(
        || {
            let mut researches = HashSet::new();
            researches.insert("cheating".to_string());
            researches
        }
    );
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
    let loc_per_clicks: Signal<Decimal> = use_signal(|| constants.loc_per_clicks);
    let debug_per_clicks: Signal<Decimal> = use_signal(|| constants.debug_per_clicks);

    // bugs ratio
    let manual_bugs_ratio: Signal<Decimal> = use_signal(|| constants.manual_bugs_ratio);
    let interns_bugs_ratio: Signal<Decimal> = use_signal(|| constants.interns_bugs_ratio);
    let junior_devs_bugs_ratio: Signal<Decimal> = use_signal(|| constants.junior_devs_bugs_ratio);
    let senior_devs_bugs_ratio: Signal<Decimal> = use_signal(|| constants.senior_devs_bugs_ratio);

    // resources
    let mut loc: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let mut bugs: Signal<Decimal> = use_signal(|| Decimal::ZERO);

    // producers
    let mut interns: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let interns_loc_dt: Signal<Decimal> = use_signal(|| constants.interns_loc_dt);
    let mut junior_devs: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let junior_devs_loc_dt: Signal<Decimal> = use_signal(|| constants.junior_devs_loc_dt);
    let mut senior_devs: Signal<Decimal> = use_signal(|| Decimal::ZERO);
    let senior_devs_loc_dt: Signal<Decimal> = use_signal(|| constants.senior_devs_loc_dt);
    let mut retired_devs: Signal<Decimal> = use_signal(|| Decimal::ZERO);

    // placeholder
    let placeholder: Signal<Decimal> = use_signal(|| Decimal::ZERO);

    // promotions & retirement
    // interns -> junior devs
    let interns_promotion_ratio_dt: Signal<Decimal> = use_signal(|| constants.interns_promotion_ratio_dt);
    // junior devs -> senior_devs
    let junior_devs_promotion_ratio_dt: Signal<Decimal> = use_signal(|| constants.junior_devs_promotion_ratio_dt);
    // senior_devs -> retired_devs
    let senior_devs_retirement_ratio_dt: Signal<Decimal> = use_signal(|| constants.senior_devs_retirement_ratio_dt);

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
                &constants.interns_loc_base_cost,
                &constants.interns_loc_growth_rate,
                &interns(),
            );
            // must be computed before incrementing junior_devs
            let manual_junior_devs_loc_cost = sum_geometric_series(
                &manual_junior_devs,
                &constants.junior_devs_loc_base_cost,
                &constants.junior_devs_loc_growth_rate,
                &junior_devs(),
            );
            // must be computed before incrementing senior_devs
            let manual_senior_devs_loc_cost = sum_geometric_series(
                &manual_senior_devs,
                &constants.senior_devs_loc_base_cost,
                &constants.senior_devs_loc_growth_rate,
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
                    Metrics {
                        researched: researched,
                        loc_dt: loc_dt,
                        bugs_dt: bugs_dt,
                        dt: dt,
                    }
                }
                if loc() > Decimal::ZERO {
                    Resources {
                        loc: loc,
                        bugs: bugs,
                        interns: interns,
                        junior_devs: junior_devs,
                        senior_devs: senior_devs,
                        retired_devs: retired_devs,
                    }
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
                        researched: researched,
                        theme: theme,
                        require: Some("toggle_theme".to_string()),
                    }
                    RepeatableAction{
                        logs: logs,
                        researched: researched,
                        clicks: interns_clicks,
                        loc: loc,
                        require: Some("internship".to_string()),
                        produced: interns,
                        button_name: "hire intern",
                        debug_message: "hiring an intern...",
                        description: "Produces loc, and bugs",
                        loc_base_cost: constants.interns_loc_base_cost,
                        loc_growth_rate: constants.interns_loc_growth_rate,
                    }
                    RepeatableAction{
                        logs: logs,
                        researched: researched,
                        clicks: junior_devs_clicks,
                        loc: loc,
                        require: Some("junior_position".to_string()),
                        produced: junior_devs,
                        button_name: "hire junior dev",
                        debug_message: "hiring a junior dev...",
                        description: "Produces loc, and bugs",
                        loc_base_cost: constants.junior_devs_loc_base_cost,
                        loc_growth_rate: constants.junior_devs_loc_growth_rate,
                    }
                    RepeatableAction{
                        logs: logs,
                        researched: researched,
                        clicks: senior_devs_clicks,
                        loc: loc,
                        require: Some("senior_position".to_string()),
                        produced: senior_devs,
                        button_name: "hire senior dev",
                        debug_message: "hiring a senior dev...",
                        description: "Produces loc, and bugs",
                        loc_base_cost: constants.senior_devs_loc_base_cost,
                        loc_growth_rate: constants.senior_devs_loc_growth_rate,
                    }
                    RepeatableAction{
                        logs: logs,
                        researched: researched,
                        clicks: rmrf_clicks,
                        loc: loc,
                        require: Some("rmrf".to_string()),
                        produced: placeholder,
                        button_name: "rm -rf",
                        debug_message: "running rm -rf...",
                        description: "Wipe out all loc and bugs",
                        loc_base_cost: Decimal::ZERO,
                        loc_growth_rate: Decimal::ONE,
                    }
                    if researched().contains("cheating") {
                        CheatAction{
                            logs: logs,
                            value: loc,
                            button_name: "cheat loc",
                            debug_message: "cheating loc...",
                        }
                        CheatAction{
                            logs: logs,
                            value: bugs,
                            button_name: "cheat bugs",
                            debug_message: "cheating bugs...",
                        }
                        CheatAction{
                            logs: logs,
                            value: interns,
                            button_name: "cheat interns",
                            debug_message: "cheating interns...",
                        }
                        CheatAction{
                            logs: logs,
                            value: junior_devs,
                            button_name: "cheat junior devs",
                            debug_message: "cheating junior devs...",
                        }
                        CheatAction{
                            logs: logs,
                            value: senior_devs,
                            button_name: "cheat senior devs",
                            debug_message: "cheating senior devs...",
                        }
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
                    }
                }
                div { // vertical
                    class: "researches",
                    ResearchOnce{
                        logs: logs,
                        researched: researched,
                        loc: loc,
                        research_name: "internship",
                        button_name: "research internship",
                        debug_message: "intership researched",
                        description: "Allow hiring interns, who produce loc and bugs automaticaly.",
                        loc_cost: constants.research_internship_loc_cost,
                    }
                    ResearchOnce{
                        logs: logs,
                        researched: researched,
                        loc: loc,
                        research_name: "code_metrics",
                        button_name: "research code metrics",
                        debug_message: "code metrics researched",
                        description: "Display LOC/s and bugs/s.",
                        loc_cost: constants.research_code_metrics_loc_cost,
                    }
                    ResearchOnce{
                        logs: logs,
                        researched: researched,
                        loc: loc,
                        research_name: "rmrf",
                        button_name: "learn rm -rf",
                        debug_message: "rm -rf researched",
                        description: "For desperate situations, allow using rm-rf command",
                        loc_cost: constants.research_rmrf_loc_cost,
                    }
                    ResearchOnce{
                        logs: logs,
                        researched: researched,
                        loc: loc,
                        require: Some("internship".to_string()),
                        research_name: "interns_promotion",
                        button_name: "promote interns",
                        debug_message: "interns promotion researched",
                        description: "Allow interns to be promoted to junior devs",
                        loc_cost: constants.research_interns_promotion_loc_cost,
                    }
                    ResearchOnce{
                        logs: logs,
                        researched: researched,
                        loc: loc,
                        require: Some("interns_promotion".to_string()),
                        research_name: "junior_devs_promotion",
                        button_name: "promote junior devs",
                        debug_message: "junior devs promotion researched",
                        description: "Allow junior devs to be promoted to senior devs",
                        loc_cost: constants.research_junior_devs_promotion_loc_cost,
                    }
                    ResearchOnce{
                        logs: logs,
                        researched: researched,
                        loc: loc,
                        research_name: "toggle_theme",
                        button_name: "install theme",
                        debug_message: "toggle theme researched",
                        description: "Allow toggling theme",
                        loc_cost: constants.research_toggle_theme_loc_cost,
                    }
                }
            }
        }
    }
}
