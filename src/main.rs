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
mod speedrun;

use simple_logs::{SimpleLogs, Logs};
use repeatable_action::RepeatableAction;
use toggle_theme_action::ToggleThemeAction;
use research_once::ResearchOnce;

use std::collections::{HashSet};
use web_time::{Instant};

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
use crate::speedrun::Speedrun;

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
    let mut researched: Signal<HashSet<String>> = use_signal(
        || {
            let mut researches = HashSet::new();
            researches.insert("cheating".to_string());
            researches
        }
    );
    let theme: Signal<Theme> = use_signal(|| Theme::LightTheme);
    let speedrun_start: Signal<Option<Instant>> = use_signal(|| None);
    let mut current_time: Signal<Instant> = use_signal(Instant::now);

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
    let mut interns_loc_dt: Signal<Decimal> = use_signal(|| constants.interns_loc_dt);
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

            // multipliers
            if researched().contains("syntax_coloring_multiplier_alias") {
                *interns_loc_dt.write() *= constants.research_syntax_coloring_multiplier;
                researched.write().remove("syntax_coloring_multiplier_alias");
            }

            // loc produced by devs
            let auto_loc = (
                interns() * interns_loc_dt()
                    + junior_devs() * junior_devs_loc_dt()
                    + senior_devs() * senior_devs_loc_dt()
            ) * dt();
            // bugs produced by devs
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

            // update current time
            *current_time.write() = Instant::now();

            // sleep before next tick
            sleep(std::time::Duration::from_millis(dt_milliseconds)).await;
        }
    });

    let research_rendered = vec![
        ("toggle_theme", "install theme", "Allow toggling theme", constants.research_toggle_theme_loc_cost, None, None),
        ("internship", "research internship", "Allow hiring interns, who produce loc and bugs automatically.", constants.research_internship_loc_cost, None, None),
        ("junior_devs_position", "research junior devs", "Allow hiring junior devs, who produce loc and bugs automatically.", constants.research_junior_devs_position_loc_cost, Some("internship".to_string()), None),
        ("senior_devs_position", "research senior devs", "Allow hiring senior devs, who produce loc and bugs automatically.", constants.research_senior_devs_position_loc_cost, Some("junior_devs_position".to_string()), None),
        ("code_metrics", "research code metrics", "Display LOC/s and bugs/s.", constants.research_code_metrics_loc_cost, None, None),
        ("speedrun", "research speedrun", "Display progress and real time timer", constants.research_speedrun_loc_cost, None, None),
        ("logs", "research logs", "Display logs", constants.research_logs_loc_cost, None, None),
        ("rmrf", "learn rm -rf", "For desperate situations, allow using rm-rf command", constants.research_rmrf_loc_cost, None, None),
        ("interns_promotion", "promote interns", "Allow interns to be promoted to junior devs", constants.research_interns_promotion_loc_cost, Some("internship".to_string()), None),
        ("junior_devs_promotion", "promote junior devs", "Allow junior devs to be promoted to senior devs", constants.research_junior_devs_promotion_loc_cost, Some("interns_promotion".to_string()), None),
        ("toggle_theme", "install theme", "Allow toggling theme", constants.research_toggle_theme_loc_cost, None, None),
        ("syntax_coloring_multiplier", "install syntax coloring", "Boost interns loc/s x{constants.research_syntax_coloring_multiplier}", constants.research_syntax_coloring_multiplier_loc_cost, None, Some("syntax_coloring_multiplier_alias".to_string())),
    ].into_iter().map(|(research_name, button_name, description, loc_cost, require, alias)|
         rsx! {
            ResearchOnce{
                logs: logs,
                researched: researched,
                loc: loc,
                require: require,
                research_name: research_name,
                research_alias: alias,
                button_name: button_name,
                debug_message: format!("{} researched", research_name),
                description: description,
                loc_cost: loc_cost,
                quest: false,
            }
        },
    );

    let quests_rendered = vec![
        ("hello_world", "code hello world", "Your 1st program",  None, constants.quest_hello_world_loc_cost),
        ("fizz_buzz", "code Fizzbuzz", "Your 2nd program",  Some("hello_world".to_string()), constants.quest_fizz_buzz_loc_cost),
        ("calculator", "code calculator", "Your 3rd program",  Some("fizz_buzz".to_string()), constants.quest_calculator_loc_cost),
        ("game_of_life", "code game of life", "?",  Some("calculator".to_string()), constants.quest_game_of_life_loc_cost),
        ("text_editor", "code a text editor", "?",  Some("game_of_life".to_string()), constants.quest_text_editor_loc_cost),
        ("physics_engine", "code a physics engine", "?",  Some("text_editor".to_string()), constants.quest_physics_engine_loc_cost),
        ("bacteria", "simulate a bacteria", "?",  Some("physics_engine".to_string()), constants.quest_bacteria_loc_cost),
        ("browser", "code a browser", "?",  Some("bacteria".to_string()), constants.quest_browser_loc_cost),
        ("kernel", "code a kernel", "?",  Some("browser".to_string()), constants.quest_kernel_loc_cost),
        ("mouse", "simulate a mouse", "?",  Some("kernel".to_string()), constants.quest_mouse_loc_cost),
        ("human_brain", "simulate a human brain", "?",  Some("mouse".to_string()), constants.quest_human_brain_loc_cost),
        ("economy", "simulate the economy", "?",  Some("human_brain".to_string()), constants.quest_economy_loc_cost),
        ("climate", "simulate the climate", "?",  Some("economy".to_string()), constants.quest_climate_loc_cost),
        ("earth", "simulate the Earth", "?",  Some("climate".to_string()), constants.quest_earth_loc_cost),
        ("solar_system", "simulate the solar system", "?",  Some("earth".to_string()), constants.quest_solar_system_loc_cost),
        ("universe", "simulate the universe", "?",  Some("solar_system".to_string()), constants.quest_universe_loc_cost),
        ("differentiation", "differentiate the simulation", "?",  Some("universe".to_string()), constants.quest_differentiation_loc_cost),
    ].into_iter().map(|(name, button_name, description, require, loc_cost)|
        rsx! {
            ResearchOnce{
                logs: logs,
                researched: researched,
                loc: loc,
                require: require,
                research_name: name,
                button_name: button_name,
                debug_message: button_name,
                description: description,
                loc_cost: loc_cost,
                quest: true,
            }
        }
    );

    rsx! {
        div { // vertical
            class: "everything",
            Logs {
                researched: researched,
                logs: logs,
            }
            div { // vertical
                class: "metrics",
                Speedrun {
                    researched: researched,
                    loc: loc,
                    speedrun_start: speedrun_start,
                    current_time: current_time,
                    max_loc: constants.quest_differentiation_loc_cost,
                }
                Metrics {
                    researched: researched,
                    loc_dt: loc_dt,
                    bugs_dt: bugs_dt,
                    dt: dt,
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
                        speedrun_start: speedrun_start,
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
                        require: Some("junior_devs_position".to_string()),
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
                        require: Some("senior_devs_position".to_string()),
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
                    {research_rendered}
                }
                div { // vertical
                    class: "quests",
                    {quests_rendered}
                }
            }
        }
    }
}
