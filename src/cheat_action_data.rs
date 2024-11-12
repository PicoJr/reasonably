#![allow(non_snake_case)]
use crate::cheat_action::CheatAction;
use crate::constants::Research;
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus::prelude::{Signal, Writable};

use crate::state::State;

#[component]
pub(crate) fn CheatActions(mut state: Signal<State>) -> Element {
    rsx! {
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
}
