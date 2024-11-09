#![allow(non_snake_case)]
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;
use crate::constants::{Clicks, Research};

use crate::format_decimal::format_decimal_loc;
use crate::state::State;

#[component]
pub(crate) fn RepeatableAction(
    mut state: Signal<State>,
    clicks: Clicks,
    require: Option<Research>,
    produced: Option<Decimal>,
    button_name: String,
    debug_message: String,
    description: String,
    loc_base_cost: Decimal,
    loc_growth_rate: Decimal,
) -> Element {
    let requirements_met = require.map_or_else(
        || true,
        |research_name_required| state.read().researched.contains(&research_name_required)
    );
    let clicks_value = match clicks {
        Clicks::Code => state.read().code_clicks,
        Clicks::Debug => state.read().debug_clicks,
        Clicks::HireInterns => state.read().interns_clicks,
        Clicks::HireJuniorDevs => state.read().junior_devs_clicks,
        Clicks::HireSeniorDevs => state.read().senior_devs_clicks,
        Clicks::HireHRs => state.read().hrs_clicks,
        Clicks::HirePMs => state.read().pms_clicks,
        Clicks::Rmrf => state.read().rmrf_clicks,
    };
    let new_instances = if let Some(produced) = produced {
        produced + clicks_value
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
                    match clicks {
                        Clicks::Code => state.write().code_clicks += Decimal::ONE,
                        Clicks::Debug => state.write().debug_clicks += Decimal::ONE,
                        Clicks::HireInterns => state.write().interns_clicks += Decimal::ONE,
                        Clicks::HireJuniorDevs => state.write().junior_devs_clicks += Decimal::ONE,
                        Clicks::HireSeniorDevs => state.write().senior_devs_clicks += Decimal::ONE,
                        Clicks::HireHRs => state.write().hrs_clicks += Decimal::ONE,
                        Clicks::HirePMs => state.write().pms_clicks += Decimal::ONE,
                        Clicks::Rmrf => state.write().rmrf_clicks += Decimal::ONE,
                    };
                    state.write().logs.log(debug_message.as_str())
                }
                , {button_name} }
            }
        }
    }
}
