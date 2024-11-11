#![allow(non_snake_case)]
use break_infinity::{Decimal, sum_geometric_series};
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;
use crate::constants::{GameConstants, Research};
use crate::repeatable_action::RepeatableAction;

use crate::state::State;

#[component]
pub(crate) fn RepeatableActions(
    mut state: Signal<State>,
    constants: GameConstants,
) -> Element {
    rsx! {
        RepeatableAction{
            state: state,
            require: Some(Research::Internship),
            produced: Some(state.read().manual_interns),
            button_name: "hire intern",
            debug_message: "hire intern",
            description: "Produces loc, and bugs",
            loc_base_cost: constants.interns_loc_base_cost,
            loc_growth_rate: constants.interns_loc_growth_rate,
            action: move |mut s: Signal<State>| {
                s.write().manual_interns += Decimal::ONE;
                let manual_interns_loc_cost = sum_geometric_series(
                    &Decimal::ONE,
                    &constants.interns_loc_base_cost,
                    &constants.interns_loc_growth_rate,
                    &state.read().manual_interns,
                );
                s.write().loc -= manual_interns_loc_cost;
            },
        }
        RepeatableAction{
            state: state,
            require: Some(Research::JuniorDevsPosition),
            produced: Some(state.read().manual_junior_devs),
            button_name: "hire junior devs",
            debug_message: "hire junior devs",
            description: "Produces loc, and bugs",
            loc_base_cost: constants.junior_devs_loc_base_cost,
            loc_growth_rate: constants.junior_devs_loc_growth_rate,
            action: move |mut s: Signal<State>| {
                s.write().manual_junior_devs += Decimal::ONE;
                let manual_junior_devs_loc_cost = sum_geometric_series(
                    &Decimal::ONE,
                    &constants.junior_devs_loc_base_cost,
                    &constants.junior_devs_loc_growth_rate,
                    &state.read().manual_junior_devs,
                );
                s.write().loc -= manual_junior_devs_loc_cost;
            },
        }
        RepeatableAction{
            state: state,
            require: Some(Research::SeniorDevsPosition),
            produced: Some(state.read().manual_senior_devs),
            button_name: "hire senior devs",
            debug_message: "hire senior devs",
            description: "Produces loc, and bugs",
            loc_base_cost: constants.senior_devs_loc_base_cost,
            loc_growth_rate: constants.senior_devs_loc_growth_rate,
            action: move |mut s: Signal<State>| {
                s.write().manual_senior_devs += Decimal::ONE;
                let manual_senior_devs_loc_cost = sum_geometric_series(
                    &Decimal::ONE,
                    &constants.senior_devs_loc_base_cost,
                    &constants.senior_devs_loc_growth_rate,
                    &state.read().manual_senior_devs,
                );
                s.write().loc -= manual_senior_devs_loc_cost;
            },
        }
        RepeatableAction{
            state: state,
            require: Some(Research::HumanResources),
            produced: Some(state.read().manual_hrs),
            button_name: "hire HR",
            debug_message: "hire HR",
            description: "Hire devs",
            loc_base_cost: constants.hrs_loc_base_cost,
            loc_growth_rate: constants.hrs_loc_growth_rate,
            action: move |mut s: Signal<State>| {
                s.write().manual_hrs += Decimal::ONE;
                let manual_hrs_loc_cost = sum_geometric_series(
                    &Decimal::ONE,
                    &constants.hrs_loc_base_cost,
                    &constants.hrs_loc_growth_rate,
                    &state.read().manual_hrs,
                );
                s.write().loc -= manual_hrs_loc_cost;
            },
        }
        RepeatableAction{
            state: state,
            require: Some(Research::ProjectManagement),
            produced: Some(state.read().manual_pms),
            button_name: "hire PM",
            debug_message: "hire PN",
            description: "Convert bugs to features",
            loc_base_cost: constants.pms_loc_base_cost,
            loc_growth_rate: constants.pms_loc_growth_rate,
            action: move |mut s: Signal<State>| {
                s.write().manual_pms += Decimal::ONE;
                let manual_pms_loc_cost = sum_geometric_series(
                    &Decimal::ONE,
                    &constants.pms_loc_base_cost,
                    &constants.pms_loc_growth_rate,
                    &state.read().manual_pms,
                );
                s.write().loc -= manual_pms_loc_cost;
            },
        }
        RepeatableAction{
            state: state,
            require: Some(Research::Rmrf),
            produced: None,
            button_name: "rm -rf",
            debug_message: "rm -rf",
            description: "Wipe all loc and bugs",
            loc_base_cost: Decimal::ZERO,
            loc_growth_rate: Decimal::ONE,
            action: move |mut s: Signal<State>| {
                s.write().loc = Decimal::ZERO;
                s.write().bugs = Decimal::ZERO;
            },
        }
    }
}
