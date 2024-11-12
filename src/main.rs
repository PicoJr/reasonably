#![allow(non_snake_case)]

mod cheat_action;
mod cheat_action_data;
mod constants;
mod format_decimal;
mod metrics;
mod repeatable_action;
mod repeatable_action_data;
mod research_data;
mod research_once;
mod resources;
mod simple_action;
mod simple_logs;
mod speedrun;
mod state;
mod toggle_theme_action;

use research_once::ResearchOnce;
use simple_logs::Logs;
use toggle_theme_action::ToggleThemeAction;

use break_infinity::Decimal;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use crate::cheat_action_data::CheatActions;
use crate::constants::{GameConstants, Research};
use crate::metrics::Metrics;
use crate::repeatable_action_data::RepeatableActions;
use crate::research_data::Researches;
use crate::resources::Resources;
use crate::simple_action::SimpleAction;
use crate::speedrun::Speedrun;
use crate::state::State;
use async_std::task::sleep;

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
        (
            Research::HelloWorld,
            "code hello world",
            "Your 1st program",
            None,
            constants.quest_hello_world_loc_cost,
        ),
        (
            Research::FizzBuzz,
            "code Fizzbuzz",
            "Your 2nd program",
            Some(Research::HelloWorld),
            constants.quest_fizz_buzz_loc_cost,
        ),
        (
            Research::Calculator,
            "code calculator",
            "Your 3rd program",
            Some(Research::FizzBuzz),
            constants.quest_calculator_loc_cost,
        ),
        (
            Research::GameOfLife,
            "code game of life",
            "?",
            Some(Research::Calculator),
            constants.quest_game_of_life_loc_cost,
        ),
        (
            Research::TextEditor,
            "code a text editor",
            "?",
            Some(Research::GameOfLife),
            constants.quest_text_editor_loc_cost,
        ),
        (
            Research::PhysicsEngine,
            "code a physics engine",
            "?",
            Some(Research::TextEditor),
            constants.quest_physics_engine_loc_cost,
        ),
        (
            Research::Bacteria,
            "simulate a bacteria",
            "?",
            Some(Research::PhysicsEngine),
            constants.quest_bacteria_loc_cost,
        ),
        (
            Research::Browser,
            "code a browser",
            "?",
            Some(Research::Bacteria),
            constants.quest_browser_loc_cost,
        ),
        (
            Research::Kernel,
            "code a kernel",
            "?",
            Some(Research::Browser),
            constants.quest_kernel_loc_cost,
        ),
        (
            Research::Mouse,
            "simulate a mouse",
            "?",
            Some(Research::Kernel),
            constants.quest_mouse_loc_cost,
        ),
        (
            Research::HumanBrain,
            "simulate a human brain",
            "?",
            Some(Research::Mouse),
            constants.quest_human_brain_loc_cost,
        ),
        (
            Research::Economy,
            "simulate the economy",
            "?",
            Some(Research::HumanBrain),
            constants.quest_economy_loc_cost,
        ),
        (
            Research::Climate,
            "simulate the climate",
            "?",
            Some(Research::Economy),
            constants.quest_climate_loc_cost,
        ),
        (
            Research::Earth,
            "simulate the Earth",
            "?",
            Some(Research::Climate),
            constants.quest_earth_loc_cost,
        ),
        (
            Research::SolarSystem,
            "simulate the solar system",
            "?",
            Some(Research::Earth),
            constants.quest_solar_system_loc_cost,
        ),
        (
            Research::Universe,
            "simulate the universe",
            "?",
            Some(Research::SolarSystem),
            constants.quest_universe_loc_cost,
        ),
        (
            Research::Differentiation,
            "differentiate the simulation",
            "?",
            Some(Research::Universe),
            constants.quest_differentiation_loc_cost,
        ),
    ]
    .into_iter()
    .map(|(name, button_name, description, require, loc_cost)| {
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
    });

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
                    RepeatableActions {
                        state: state,
                        constants: constants.clone(),
                    }
                    CheatActions {
                        state: state,
                    }
                }
                Researches {
                    state: state,
                    constants: constants.clone(),
                }
                div { // vertical
                    class: "quests",
                    {quests_rendered}
                }
            }
        }
    }
}
