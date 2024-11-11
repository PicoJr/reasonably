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
            state.write().update(dt_seconds);
            // sleep before next tick
            sleep(std::time::Duration::from_millis(dt_milliseconds)).await;
        }
    });

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
                    ResearchOnce{
                        state: state,
                        require: None,
                        research_name: Research::ToggleTheme,
                        button_name: "Install theme",
                        debug_message: format!("{:?} researched", Research::ToggleTheme),
                        description: "allow toggling theme",
                        loc_cost: constants.research_toggle_theme_loc_cost,
                        quest: false,
                    }
                    ResearchOnce{
                        state: state,
                        require: None,
                        research_name: Research::Internship,
                        button_name: "Research internship",
                        debug_message: format!("{:?} researched", Research::Internship),
                        description: "allow hiring interns, who produce loc and bugs automatically",
                        loc_cost: constants.research_internship_loc_cost,
                        quest: false,
                    }
                    ResearchOnce{
                        state: state,
                        require: Some(Research::Internship),
                        research_name: Research::JuniorDevsPosition,
                        button_name: "Research junior devs",
                        debug_message: format!("{:?} researched", Research::JuniorDevsPosition),
                        description: "allow hiring junior devs, who produce loc and bugs automatically",
                        loc_cost: constants.research_junior_devs_position_loc_cost,
                        quest: false,
                    }
                    ResearchOnce{
                        state: state,
                        require: Some(Research::JuniorDevsPosition),
                        research_name: Research::SeniorDevsPosition,
                        button_name: "research senior devs",
                        debug_message: format!("{:?} researched", Research::SeniorDevsPosition),
                        description: "Allow hiring senior devs, who produce loc and bugs automatically",
                        loc_cost: constants.research_senior_devs_position_loc_cost,
                        quest: false,
                    }
                    ResearchOnce{
                        state: state,
                        require: None,
                        research_name: Research::CodeMetrics,
                        button_name: "research code metrics",
                        debug_message: format!("{:?} researched", Research::CodeMetrics),
                        description: "Display LOC/s and bugs/s",
                        loc_cost: constants.research_code_metrics_loc_cost,
                        quest: false,
                    }
                    ResearchOnce{
                        state: state,
                        require: None,
                        research_name: Research::Speedrun,
                        button_name: "research speedrun",
                        debug_message: format!("{:?} researched", Research::Speedrun),
                        description: "Display progress bar and real time timer",
                        loc_cost: constants.research_speedrun_loc_cost,
                        quest: false,
                    }
                    ResearchOnce{
                        state: state,
                        require: None,
                        research_name: Research::Logs,
                        button_name: "research logs",
                        debug_message: format!("{:?} researched", Research::Logs),
                        description: "Display logs",
                        loc_cost: constants.research_logs_loc_cost,
                        quest: false,
                    }
                    ResearchOnce{
                        state: state,
                        require: None,
                        research_name: Research::Rmrf,
                        button_name: "learn rm -rf",
                        debug_message: format!("{:?} researched", Research::Rmrf),
                        description: "For desperate situations, allow using rm -rf command",
                        loc_cost: constants.research_rmrf_loc_cost,
                        quest: false,
                    }
                    ResearchOnce{
                        state: state,
                        require: Some(Research::JuniorDevsPosition),
                        research_name: Research::InternsPromotion,
                        button_name: "promote interns",
                        debug_message: format!("{:?} researched", Research::InternsPromotion),
                        description: "Allow interns to be promoted to junior devs",
                        loc_cost: constants.research_interns_promotion_loc_cost,
                        quest: false,
                    }
                    ResearchOnce{
                        state: state,
                        require: Some(Research::SeniorDevsPosition),
                        research_name: Research::JuniorDevsPromotion,
                        button_name: "promote junior devs",
                        debug_message: format!("{:?} researched", Research::JuniorDevsPromotion),
                        description: "Allow junior devs to be promoted to senior devs",
                        loc_cost: constants.research_junior_devs_promotion_loc_cost,
                        quest: false,
                    }
                    ResearchOnce{
                        state: state,
                        require: Some(Research::Internship),
                        research_name: Research::SyntaxColoringMultiplier,
                        button_name: "install syntax coloring",
                        debug_message: format!("{:?} researched", Research::SyntaxColoringMultiplier),
                        description: "Boost interns locs/s x2",
                        loc_cost: constants.research_syntax_coloring_multiplier_loc_cost,
                        quest: false,
                        action: move |mut s: Signal<State>| {
                            s.write().interns_loc_dt *= constants.research_syntax_coloring_multiplier;
                        },
                    }
                    ResearchOnce{
                        state: state,
                        require: Some(Research::SeniorDevsPosition),
                        research_name: Research::HumanResources,
                        button_name: "research human resources",
                        debug_message: format!("{:?} researched", Research::HumanResources),
                        description: "Allow hiring HR, who hire devs",
                        loc_cost: constants.research_human_resources_loc_cost,
                        quest: false,
                    }
                    ResearchOnce{
                        state: state,
                        require: Some(Research::SeniorDevsPosition),
                        research_name: Research::ProjectManagement,
                        button_name: "research project management",
                        debug_message: format!("{:?} researched", Research::ProjectManagement),
                        description: "Allow hiring PM, who convert bugs to features",
                        loc_cost: constants.research_project_management_loc_cost,
                        quest: false,
                    }
                    ResearchOnce{
                        state: state,
                        require: Some(Research::ProjectManagement),
                        research_name: Research::ManagementCareer,
                        button_name: "research management career",
                        debug_message: format!("{:?} researched", Research::ManagementCareer),
                        description: "Instead of retiring, some senior devs will become PMs",
                        loc_cost: constants.research_management_career_loc_cost,
                        quest: false,
                        action: move |mut s: Signal<State>| {
                            let retirement_ratio_dt = s.read().senior_devs_retirement_ratio_dt;
                            s.write().senior_devs_retirement_ratio_dt = retirement_ratio_dt * (Decimal::ONE - constants.senior_devs_management_career_ratio);
                            s.write().senior_devs_management_ratio_dt = retirement_ratio_dt * constants.senior_devs_management_career_ratio;
                        },
                    }
                }
                div { // vertical
                    class: "quests",
                    {quests_rendered}
                }
            }
        }
    }
}
