#![allow(non_snake_case)]
use crate::constants::{GameConstants, Research};
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus::prelude::{Signal, Writable};

use crate::research_once::ResearchOnce;
use crate::state::State;

#[component]
pub(crate) fn Quests(mut state: Signal<State>, constants: GameConstants) -> Element {
    rsx! {
        div { // vertical
            class: "quests",
            ResearchOnce{
                state: state,
                require: None,
                research_name: Research::HelloWorld,
                button_name: "code hello world",
                description: "Your 1st program",
                loc_cost: constants.quest_hello_world_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::HelloWorld),
                research_name: Research::FizzBuzz,
                button_name: "code Fizzbuzz",
                description: "Your 2nd program",
                loc_cost: constants.quest_fizz_buzz_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::FizzBuzz),
                research_name: Research::Calculator,
                button_name: "code calculator",
                description: "Your 3rd program",
                loc_cost: constants.quest_calculator_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::Calculator),
                research_name: Research::GameOfLife,
                button_name: "code game of life",
                description: "?",
                loc_cost: constants.quest_game_of_life_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::GameOfLife),
                research_name: Research::TextEditor,
                button_name: "code a text editor",
                description: "?",
                loc_cost: constants.quest_text_editor_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::TextEditor),
                research_name: Research::PhysicsEngine,
                button_name: "code a physics engine",
                description: "?",
                loc_cost: constants.quest_physics_engine_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::PhysicsEngine),
                research_name: Research::Bacteria,
                button_name: "simulate a bacteria",
                description: "?",
                loc_cost: constants.quest_bacteria_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::Bacteria),
                research_name: Research::Browser,
                button_name: "code a browser",
                description: "?",
                loc_cost: constants.quest_browser_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::Browser),
                research_name: Research::Kernel,
                button_name: "code a kernel",
                description: "?",
                loc_cost: constants.quest_kernel_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::Kernel),
                research_name: Research::Mouse,
                button_name: "simulate a mouse",
                description: "?",
                loc_cost: constants.quest_mouse_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::Mouse),
                research_name: Research::HumanBrain,
                button_name: "simulate a human brain",
                description: "?",
                loc_cost: constants.quest_human_brain_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::HumanBrain),
                research_name: Research::Economy,
                button_name: "simulate the economy",
                description: "?",
                loc_cost: constants.quest_economy_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::Economy),
                research_name: Research::Climate,
                button_name: "simulate the climate",
                description: "?",
                loc_cost: constants.quest_climate_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::Climate),
                research_name: Research::Earth,
                button_name: "simulate the Earth",
                description: "?",
                loc_cost: constants.quest_earth_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::Earth),
                research_name: Research::SolarSystem,
                button_name: "simulate the solar system",
                description: "?",
                loc_cost: constants.quest_solar_system_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::SolarSystem),
                research_name: Research::Universe,
                button_name: "simulate the universe",
                description: "?",
                loc_cost: constants.quest_universe_loc_cost,
                quest: true,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::Universe),
                research_name: Research::Differentiation,
                button_name: "differentiate the simulation",
                description: "?",
                loc_cost: constants.quest_differentiation_loc_cost,
                quest: true,
            }
        }
    }
}
