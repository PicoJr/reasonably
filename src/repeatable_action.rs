#![allow(non_snake_case)]
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;
use crate::constants::{Research};

use crate::format_decimal::format_decimal_loc;
use crate::state::State;

#[component]
pub(crate) fn RepeatableAction(
    mut state: Signal<State>,
    require: Option<Research>,
    produced: Option<Decimal>,
    button_name: String,
    debug_message: String,
    description: String,
    loc_base_cost: Decimal,
    loc_growth_rate: Decimal,
    action: EventHandler<Signal<State>>,
) -> Element {
    let requirements_met = require.map_or_else(
        || true,
        |research_name_required| state.read().researched.contains(&research_name_required)
    );
    let new_instances = if let Some(produced) = produced {
        produced + Decimal::new(1.0)
    } else {
        Decimal::ZERO
    };
    let loc_cost = loc_base_cost * loc_growth_rate.pow(&new_instances);
    let disabled = state.read().loc < loc_cost;
    rsx! {
        if requirements_met {
            div {
                class: "repeatable-action",
                p {"{description}"}
                p {"Cost {format_decimal_loc(loc_cost)}"}
                button {
                    disabled: disabled,
                    class: "repeatable-action-button",
                    onclick: move |_| {
                        action.call(state);
                        state.write().logs.log(debug_message.as_str())
                }
                , {button_name} }
            }
        }
    }
}
