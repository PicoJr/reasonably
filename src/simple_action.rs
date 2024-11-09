#![allow(non_snake_case)]

use web_time::{Instant};
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::Signal;
use dioxus::prelude::*;
use crate::constants::Clicks;
use crate::state::State;

#[component]
pub(crate) fn SimpleAction(
    mut state: Signal<State>,
    clicks: Clicks,
    button_name: String,
) -> Element {
    let log = format!("clicked on {} button", button_name.clone());
    rsx! {
        button {
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
                state.write().logs.log(log.as_str());
                if state.read().speedrun_start.is_none() {
                    state.write().speedrun_start = Some(Instant::now());
                }
        }
        , {button_name} }
    }
}
