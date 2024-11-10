#![allow(non_snake_case)]

mod simple_logs;
mod repeatable_action;
mod toggle_theme_action;
mod research_once;
mod format_decimal;
mod constants;
mod cheat_action;
mod metrics;
mod resources;
mod speedrun;
mod state;
mod simple_action;

use simple_logs::{Logs};
use repeatable_action::RepeatableAction;
use toggle_theme_action::ToggleThemeAction;
use research_once::ResearchOnce;

use web_time::{Instant};

use break_infinity::{sum_geometric_series, Decimal};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use async_std::task::sleep;
use crate::cheat_action::CheatAction;
use crate::constants::{GameConstants, Research};
use crate::metrics::Metrics;
use crate::resources::Resources;
use crate::simple_action::SimpleAction;
use crate::speedrun::Speedrun;
use crate::state::State;

#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
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
    let mut state: Signal<State> = use_signal(|| State::new(constants.clone()));

    use_future(move || async move {
        let dt_milliseconds = 100; // real time between 2 updates
        let dt_seconds = Decimal::new(1e3 / dt_milliseconds as f64);
        loop {

            // multipliers
            if state.read().researched.contains(&Research::SyntaxColoringMultiplierAlias) {
                state.write().interns_loc_dt *= constants.research_syntax_coloring_multiplier;
                state.write().researched.remove(&Research::SyntaxColoringMultiplierAlias);
            }

            if state.read().researched.contains(&Research::ManagementCareerAlias) {
                let retirement_ratio_dt = state.read().senior_devs_retirement_ratio_dt;
                state.write().senior_devs_retirement_ratio_dt = retirement_ratio_dt * (Decimal::ONE - constants.senior_devs_management_career_ratio);
                state.write().senior_devs_management_ratio_dt = retirement_ratio_dt * constants.senior_devs_management_career_ratio;
                state.write().researched.remove(&Research::ManagementCareerAlias);
            }

            // loc produced by devs
            let auto_loc = (
                (state.read().interns + state.read().manual_interns) * state.read().interns_loc_dt
                    + (state.read().junior_devs + state.read().manual_junior_devs) * state.read().junior_devs_loc_dt
                    + (state.read().senior_devs + state.read().manual_senior_devs) * state.read().senior_devs_loc_dt
            ) * state.read().dt;
            // bugs produced by devs
            let auto_bugs = (
                (state.read().interns + state.read().manual_interns) * state.read().interns_loc_dt * state.read().interns_bugs_ratio
                    + (state.read().junior_devs + state.read().manual_junior_devs) * state.read().junior_devs_loc_dt * state.read().junior_devs_bugs_ratio
                    + (state.read().senior_devs + state.read().manual_senior_devs) * state.read().senior_devs_loc_dt * state.read().senior_devs_bugs_ratio
            ) * state.read().dt;

            // update loc, accounting all sources
            state.write().loc += auto_loc;
            // update live code metrics
            state.write().loc_dt = auto_loc * dt_seconds;

            let auto_bugs_converted_capacity = (state.read().pms + state.read().manual_pms) * state.read().pms_bugs_conversion_dt * state.read().dt;
            // make sure we do not convert more bugs than available
            let bugs_converted = state.read().bugs.min(&auto_bugs_converted_capacity);
            let bugs_delta = auto_bugs - bugs_converted;
            state.write().bugs += bugs_delta;
            state.write().bugs_dt = bugs_delta * dt_seconds;

            state.write().features += bugs_converted;
            state.write().features_dt = bugs_converted * dt_seconds;

            let auto_interns = (state.read().hrs + state.read().manual_hrs) * state.read().hrs_interns_dt * state.read().hrs_interns_quota * state.read().dt;
            let auto_junior_devs = (state.read().hrs + state.read().manual_hrs) * state.read().hrs_junior_devs_dt * state.read().hrs_junior_devs_quota * state.read().dt;
            let auto_senior_devs = (state.read().hrs + state.read().manual_hrs) * state.read().hrs_senior_devs_dt * state.read().hrs_senior_devs_quota * state.read().dt;

            // update interns, junior devs, senior devs count, accounting for all sources
            state.write().interns += auto_interns;
            state.write().junior_devs += auto_junior_devs;
            state.write().senior_devs += auto_senior_devs;
            let seniors_becoming_pms = (state.read().senior_devs + state.read().manual_senior_devs) * state.read().senior_devs_management_ratio_dt * state.read().dt;
            state.write().pms += seniors_becoming_pms;

            // handle promotions & retirement...
            let retired_seniors = (state.read().senior_devs + state.read().manual_senior_devs) * state.read().senior_devs_retirement_ratio_dt * state.read().dt;
            state.write().retired_devs += retired_seniors;
            let remaining_seniors = state.read().senior_devs * (Decimal::ONE - state.read().senior_devs_retirement_ratio_dt * state.read().dt);
            state.write().senior_devs = remaining_seniors;
            let remaining_manual_seniors =state.read().manual_senior_devs * (Decimal::ONE - state.read().senior_devs_retirement_ratio_dt * state.read().dt);
            state.write().manual_senior_devs = remaining_manual_seniors;

            if state.read().researched.contains(&Research::JuniorDevsPromotion) {
                let juniors_promoted_to_seniors = (state.read().junior_devs + state.read().manual_junior_devs) * state.read().junior_devs_promotion_ratio_dt * state.read().dt;
                state.write().senior_devs += juniors_promoted_to_seniors;
                let remaining_juniors = state.read().junior_devs * (Decimal::ONE - state.read().junior_devs_promotion_ratio_dt * state.read().dt);
                state.write().junior_devs = remaining_juniors;
                let remaining_manual_juniors = state.read().manual_junior_devs * (Decimal::ONE - state.read().junior_devs_promotion_ratio_dt * state.read().dt);
                state.write().manual_junior_devs = remaining_manual_juniors;
            }

            if state.read().researched.contains(&Research::InternsPromotion) {
                let interns_promoted_juniors = (state.read().interns + state.read().manual_interns) * state.read().interns_promotion_ratio_dt * state.read().dt;
                state.write().junior_devs += interns_promoted_juniors;
                let remaining_interns = state.read().interns * (Decimal::ONE - state.read().interns_promotion_ratio_dt * state.read().dt);
                state.write().interns = remaining_interns;
                let remaining_manual_interns = state.read().manual_interns * (Decimal::ONE - state.read().interns_promotion_ratio_dt * state.read().dt);
                state.write().manual_interns = remaining_manual_interns;
            }

            // update current time
            state.write().current_time = Instant::now();

            // sleep before next tick
            sleep(std::time::Duration::from_millis(dt_milliseconds)).await;
        }
    });

    let research_rendered = vec![
        (Research::ToggleTheme, "install theme", "Allow toggling theme", constants.research_toggle_theme_loc_cost, None, None),
        (Research::Internship, "research internship", "Allow hiring interns, who produce loc and bugs automatically.", constants.research_internship_loc_cost, None, None),
        (Research::JuniorDevsPosition, "research junior devs", "Allow hiring junior devs, who produce loc and bugs automatically.", constants.research_junior_devs_position_loc_cost, Some(Research::Internship), None),
        (Research::SeniorDevsPosition, "research senior devs", "Allow hiring senior devs, who produce loc and bugs automatically.", constants.research_senior_devs_position_loc_cost, Some(Research::JuniorDevsPosition), None),
        (Research::CodeMetrics, "research code metrics", "Display LOC/s and bugs/s.", constants.research_code_metrics_loc_cost, None, None),
        (Research::Speedrun, "research speedrun", "Display progress and real time timer", constants.research_speedrun_loc_cost, None, None),
        (Research::Logs, "research logs", "Display logs", constants.research_logs_loc_cost, None, None),
        (Research::Rmrf, "learn rm -rf", "For desperate situations, allow using rm-rf command", constants.research_rmrf_loc_cost, None, None),
        (Research::InternsPromotion, "promote interns", "Allow interns to be promoted to junior devs", constants.research_interns_promotion_loc_cost, Some(Research::Internship), None),
        (Research::JuniorDevsPromotion, "promote junior devs", "Allow junior devs to be promoted to senior devs", constants.research_junior_devs_promotion_loc_cost, Some(Research::InternsPromotion), None),
        (Research::ToggleTheme, "install theme", "Allow toggling theme", constants.research_toggle_theme_loc_cost, None, None),
        (Research::SyntaxColoringMultiplier, "install syntax coloring", "Boost interns loc/s x2", constants.research_syntax_coloring_multiplier_loc_cost, None, Some(Research::SyntaxColoringMultiplierAlias)),
        (Research::HumanResources, "research human resources", "Allow hiring HR, who hire devs", constants.research_human_resources_loc_cost, Some(Research::SeniorDevsPosition), None),
        (Research::ProjectManagement, "research project management", "Allow hiring PMs, who convert bugs to features", constants.research_project_management_loc_cost, Some(Research::HumanResources), None),
        (Research::ManagementCareer, "research management career", "Instead of retiring, some senior devs will become PMs", constants.research_management_career_loc_cost, Some(Research::ProjectManagement), Some(Research::ManagementCareerAlias)),
    ].into_iter().map(|(research_name, button_name, description, loc_cost, require, alias)|
         rsx! {
            ResearchOnce{
                state: state,
                require: require,
                research_name: research_name.clone(),
                research_alias: alias,
                button_name: button_name,
                debug_message: format!("{:?} researched", research_name.clone()),
                description: description,
                loc_cost: loc_cost,
                quest: false,
            }
        },
    );

    let quests_rendered = vec![
        (Research::HelloWorld, "code hello world", "Your 1st program",  None, constants.quest_hello_world_loc_cost),
        (Research::FizzBuzz, "code Fizzbuzz", "Your 2nd program",  Some(Research::HelloWorld), constants.quest_fizz_buzz_loc_cost),
        (Research::Calculator, "code calculator", "Your 3rd program",  Some(Research::FizzBuzz), constants.quest_calculator_loc_cost),
        (Research::GameOfLife, "code game of life", "?",  Some(Research::Calculator), constants.quest_game_of_life_loc_cost),
        (Research::TextEditor, "code a text editor", "?",  Some(Research::GameOfLife), constants.quest_text_editor_loc_cost),
        (Research::PhysicsEngine, "code a physics engine", "?",  Some(Research::TextEditor), constants.quest_physics_engine_loc_cost),
        (Research::Bacteria, "simulate a bacteria", "?",  Some(Research::PhysicsEngine), constants.quest_bacteria_loc_cost),
        (Research::Browser, "code a browser", "?",  Some(Research::Bacteria), constants.quest_browser_loc_cost),
        (Research::Kernel, "code a kernel", "?",  Some(Research::Browser), constants.quest_kernel_loc_cost),
        (Research::Mouse, "simulate a mouse", "?",  Some(Research::Kernel), constants.quest_mouse_loc_cost),
        (Research::HumanBrain, "simulate a human brain", "?",  Some(Research::Mouse), constants.quest_human_brain_loc_cost),
        (Research::Economy, "simulate the economy", "?",  Some(Research::HumanBrain), constants.quest_economy_loc_cost),
        (Research::Climate, "simulate the climate", "?",  Some(Research::Economy), constants.quest_climate_loc_cost),
        (Research::Earth, "simulate the Earth", "?",  Some(Research::Climate), constants.quest_earth_loc_cost),
        (Research::SolarSystem, "simulate the solar system", "?",  Some(Research::Earth), constants.quest_solar_system_loc_cost),
        (Research::Universe, "simulate the universe", "?",  Some(Research::SolarSystem), constants.quest_universe_loc_cost),
        (Research::Differentiation, "differentiate the simulation", "?",  Some(Research::Universe), constants.quest_differentiation_loc_cost),
    ].into_iter().map(|(name, button_name, description, require, loc_cost)|
        rsx! {
            ResearchOnce{
                state: state,
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
                state: state,
            }
            div { // vertical
                class: "metrics",
                Speedrun {
                    state: state,
                    max_loc: constants.quest_differentiation_loc_cost,
                }
                Metrics {
                    state: state,
                }
                if state.read().loc > Decimal::ZERO {
                    Resources {
                        state: state,
                    }
                }
            }
            div { // horizontal
                class: "interactions",
                div { // vertical
                    class: "repeatable-actions",
                    SimpleAction {
                        state: state,
                        button_name: "code",
                        action: move |mut s: Signal<State>| {
                            let loc_added = state.read().loc_per_clicks;
                            let bugs_added = loc_added * state.read().manual_bugs_ratio;
                            s.write().loc += loc_added;
                            s.write().bugs += bugs_added;
                        },
                    }
                    if state.read().bugs > Decimal::ZERO {
                        SimpleAction {
                            state: state,
                            button_name: "debug",
                            action: move |mut s: Signal<State>| {
                                let bugs_removed = state.read().debug_per_clicks;
                                s.write().bugs -= bugs_removed;
                            },
                        }
                    }
                    ToggleThemeAction {
                        state: state,
                    }
                    RepeatableAction{
                        state: state,
                        require: Some(Research::Internship),
                        produced: Some(state.read().manual_interns),
                        button_name: "hire intern",
                        debug_message: "hire intern",
                        description: "Produces loc, and bugs",
                        loc_base_cost: constants.interns_loc_base_cost,
                        loc_growth_rate: constants.interns_loc_growth_rate,
                        action: move |mut s: Signal<State>| {
                            s.write().manual_interns += Decimal::ONE;
                            let manual_interns_loc_cost = sum_geometric_series(
                                &Decimal::ONE,
                                &constants.interns_loc_base_cost,
                                &constants.interns_loc_growth_rate,
                                &state.read().manual_interns,
                            );
                            s.write().loc -= manual_interns_loc_cost;
                        },
                    }
                    RepeatableAction{
                        state: state,
                        require: Some(Research::JuniorDevsPosition),
                        produced: Some(state.read().manual_junior_devs),
                        button_name: "hire junior devs",
                        debug_message: "hire junior devs",
                        description: "Produces loc, and bugs",
                        loc_base_cost: constants.junior_devs_loc_base_cost,
                        loc_growth_rate: constants.junior_devs_loc_growth_rate,
                        action: move |mut s: Signal<State>| {
                            s.write().manual_junior_devs += Decimal::ONE;
                            let manual_junior_devs_loc_cost = sum_geometric_series(
                                &Decimal::ONE,
                                &constants.junior_devs_loc_base_cost,
                                &constants.junior_devs_loc_growth_rate,
                                &state.read().manual_junior_devs,
                            );
                            s.write().loc -= manual_junior_devs_loc_cost;
                        },
                    }
                    RepeatableAction{
                        state: state,
                        require: Some(Research::SeniorDevsPosition),
                        produced: Some(state.read().manual_senior_devs),
                        button_name: "hire senior devs",
                        debug_message: "hire senior devs",
                        description: "Produces loc, and bugs",
                        loc_base_cost: constants.senior_devs_loc_base_cost,
                        loc_growth_rate: constants.senior_devs_loc_growth_rate,
                        action: move |mut s: Signal<State>| {
                            s.write().manual_senior_devs += Decimal::ONE;
                            let manual_senior_devs_loc_cost = sum_geometric_series(
                                &Decimal::ONE,
                                &constants.senior_devs_loc_base_cost,
                                &constants.senior_devs_loc_growth_rate,
                                &state.read().manual_senior_devs,
                            );
                            s.write().loc -= manual_senior_devs_loc_cost;
                        },
                    }
                    RepeatableAction{
                        state: state,
                        require: Some(Research::HumanResources),
                        produced: Some(state.read().manual_hrs),
                        button_name: "hire HR",
                        debug_message: "hire HR",
                        description: "Hire devs",
                        loc_base_cost: constants.hrs_loc_base_cost,
                        loc_growth_rate: constants.hrs_loc_growth_rate,
                        action: move |mut s: Signal<State>| {
                            s.write().manual_hrs += Decimal::ONE;
                            let manual_hrs_loc_cost = sum_geometric_series(
                                &Decimal::ONE,
                                &constants.hrs_loc_base_cost,
                                &constants.hrs_loc_growth_rate,
                                &state.read().manual_hrs,
                            );
                            s.write().loc -= manual_hrs_loc_cost;
                        },
                    }
                    RepeatableAction{
                        state: state,
                        require: Some(Research::ProjectManagement),
                        produced: Some(state.read().manual_pms),
                        button_name: "hire PM",
                        debug_message: "hire PN",
                        description: "Convert bugs to features",
                        loc_base_cost: constants.pms_loc_base_cost,
                        loc_growth_rate: constants.pms_loc_growth_rate,
                        action: move |mut s: Signal<State>| {
                            s.write().manual_pms += Decimal::ONE;
                            let manual_pms_loc_cost = sum_geometric_series(
                                &Decimal::ONE,
                                &constants.pms_loc_base_cost,
                                &constants.pms_loc_growth_rate,
                                &state.read().manual_pms,
                            );
                            s.write().loc -= manual_pms_loc_cost;
                        },
                    }
                    RepeatableAction{
                        state: state,
                        require: Some(Research::Rmrf),
                        produced: None,
                        button_name: "rm -rf",
                        debug_message: "rm -rf",
                        description: "Wipe all loc and bugs",
                        loc_base_cost: Decimal::ZERO,
                        loc_growth_rate: Decimal::ONE,
                        action: move |mut s: Signal<State>| {
                            s.write().loc = Decimal::ZERO;
                            s.write().bugs = Decimal::ZERO;
                        },
                    }
                    if state.read().researched.contains(&Research::Cheating) {
                        CheatAction{
                            state: state,
                            button_name: "cheat loc",
                            debug_message: "cheating loc...",
                            action: move |mut s: Signal<State>| {
                                s.write().loc *= Decimal::new(2.0)
                            },
                        }
                        CheatAction{
                            state: state,
                            button_name: "cheat debug",
                            debug_message: "cheating debug...",
                            action: move |mut s: Signal<State>| {
                                s.write().bugs *= Decimal::new(0.5)
                            },
                        }
                        CheatAction{
                            state: state,
                            button_name: "cheat interns",
                            debug_message: "cheating interns...",
                            action: move |mut s: Signal<State>| {
                                s.write().interns *= Decimal::new(2.0)
                            },
                        }
                        CheatAction{
                            state: state,
                            button_name: "cheat junior devs",
                            debug_message: "cheating junior devs...",
                            action: move |mut s: Signal<State>| {
                                s.write().junior_devs *= Decimal::new(2.0)
                            },
                        }
                        CheatAction{
                            state: state,
                            button_name: "cheat senior devs",
                            debug_message: "cheating senior devs...",
                            action: move |mut s: Signal<State>| {
                                s.write().senior_devs *= Decimal::new(2.0)
                            },
                        }
                        CheatAction{
                            state: state,
                            button_name: "cheat dt faster",
                            debug_message: "cheating dt faster",
                            action: move |mut s: Signal<State>| {
                                s.write().dt *= Decimal::new(2.0)
                            },
                        }
                        CheatAction{
                            state: state,
                            button_name: "cheat dt slower",
                            debug_message: "cheating dt slower",
                            action: move |mut s: Signal<State>| {
                                s.write().dt *= Decimal::new(0.5)
                            },
                        }
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
