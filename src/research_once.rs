#![allow(non_snake_case)]
use std::collections::HashSet;
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;

use crate::simple_logs::SimpleLogs;
use crate::format_decimal::format_decimal_loc;

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
    quest: bool,
) -> Element {
    let (css_class, css_button_class) = if quest {
        ("quest", "quest-button")
    } else {
        ("research", "research-button")
    };
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
                class: css_class,
                p {"{description}"}
                p {"Cost {format_decimal_loc(loc_cost)}"}
                button {
                    class: css_button_class,
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
