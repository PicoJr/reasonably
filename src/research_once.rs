use std::collections::HashSet;
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;

use crate::simple_logs::SimpleLogs;

#[component]
pub(crate) fn ResearchOnce(
    mut logs: Signal<SimpleLogs>,
    mut researched: Signal<HashSet<String>>,
    mut loc: Signal<Decimal>,
    research_name: String,
    button_name: String,
    debug_message: String,
    description: String,
    loc_cost: Decimal,
) -> Element {
    let disabled = loc() < loc_cost;
    rsx! {
        div {
            class: "research",
            p {"{description}"}
            p {"Cost {loc_cost} loc"}
            button {
                class: "research-button",
                disabled: disabled,
                onclick: move |_| {
                researched.write().insert(research_name.clone()) ;
                logs.write().log(
                    &debug_message
                );
                loc -= loc_cost;
            }
            , {button_name} }
        }
    }
}
