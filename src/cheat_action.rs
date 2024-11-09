#![allow(non_snake_case)]
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;
use crate::constants::Clicks;
use crate::state::State;

#[component]
pub(crate) fn CheatAction(
    mut state: Signal<State>,
    clicks: Clicks,
    button_name: String,
    debug_message: String,
) -> Element {
    rsx! {
        button {
            class: "repeatable-action-button",
            onclick: move |_| {
            match clicks {
                Clicks::Code => state.write().loc *= Decimal::new(2.0),
                Clicks::Debug => state.write().bugs /= Decimal::new(2.0),
                Clicks::HireInterns => state.write().interns *= Decimal::new(2.0),
                Clicks::HireJuniorDevs => state.write().junior_devs *= Decimal::new(2.0),
                Clicks::HireSeniorDevs => state.write().senior_devs *= Decimal::new(2.0),
                Clicks::HireHRs => state.write().hrs *= Decimal::new(2.0),
                Clicks::HirePMs => state.write().pms *= Decimal::new(2.0),
                Clicks::Rmrf => (),
            };
            state.write().logs.log(
                debug_message.as_str()
            )
        }
        , {button_name} }
    }
}
