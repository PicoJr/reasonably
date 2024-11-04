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
    require: Option<String>,
    button_name: String,
    debug_message: String,
    description: String,
    loc_cost: Decimal,
) -> Element {
    let disabled = loc() < loc_cost;
    // only show if not researched already
    let already_researched = researched().contains(research_name.as_str());
    let requirements_met = require.map_or_else(
        || true,
        |research_name_required| researched().contains(research_name_required.as_str())
    );
    rsx! {
        if !already_researched && requirements_met {
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
}
