#![allow(non_snake_case)]
use crate::state::State;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus::prelude::{Signal, Writable};

#[component]
pub(crate) fn CheatAction(
    mut state: Signal<State>,
    button_name: String,
    debug_message: String,
    action: EventHandler<Signal<State>>,
) -> Element {
    rsx! {
        button {
            class: "repeatable-action-button",
            onclick: move |_| {
                action.call(state);
                state.write().logs.log(
                    debug_message.as_str()
                )
        }
        , {button_name} }
    }
}
