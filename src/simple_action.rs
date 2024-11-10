#![allow(non_snake_case)]

use web_time::{Instant};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::Signal;
use dioxus::prelude::*;
use crate::state::State;

#[component]
pub(crate) fn SimpleAction(
    mut state: Signal<State>,
    button_name: String,
    action: EventHandler<Signal<State>>,
) -> Element {
    let log = format!("clicked on {} button", button_name.clone());
    rsx! {
        button {
            class: "repeatable-action-button",
            onclick: move |_| {
                action.call(state);
                state.write().logs.log(log.as_str());
                if state.read().speedrun_start.is_none() {
                    state.write().speedrun_start = Some(Instant::now());
                }
        }
        , {button_name} }
    }
}
