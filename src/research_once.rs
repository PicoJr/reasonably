#![allow(non_snake_case)]
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;
use crate::constants::Research;

use crate::format_decimal::format_decimal_loc;
use crate::state::State;

#[component]
pub(crate) fn ResearchOnce(
    mut state: Signal<State>,
    research_name: Research,
    require: Option<Research>,
    button_name: String,
    debug_message: String,
    description: String,
    loc_cost: Decimal,
    quest: bool,
) -> Element {
    let (css_class, css_button_class) = if quest {
        ("quest", "quest-button")
    } else {
        ("research", "research-button")
    };
    let disabled = state.read().loc < loc_cost;
    // only show if not researched already
    let already_researched = state.read().researched.contains(&research_name);
    let requirements_met = require.map_or_else(
        || true,
        |research_name_required| state.read().researched.contains(&research_name_required)
    );
    rsx! {
        if !already_researched && requirements_met {
            div {
                class: css_class,
                p {"{description}"}
                p {"Cost {format_decimal_loc(loc_cost)}"}
                button {
                    class: css_button_class,
                    disabled: disabled,
                    onclick: move |_| {
                    state.write().researched.insert(research_name.clone());
                    state.write().logs.log(
                        &debug_message
                    );
                    state.write().loc -= loc_cost;
                }
                , {button_name} }
            }
        }
    }
}
