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
mod quest_data;

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
use crate::quest_data::Quests;

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
                Quests {
                    state: state,
                    constants: constants.clone(),
                }
            }
        }
    }
}
