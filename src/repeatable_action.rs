use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;

use crate::simple_logs::SimpleLogs;

#[component]
pub(crate) fn RepeatableAction(
    mut logs: Signal<SimpleLogs>,
    mut clicks: Signal<Decimal>,
    loc: Signal<Decimal>,
    produced: Signal<Decimal>,
    button_name: String,
    debug_message: String,
    description: String,
    loc_base_cost: Decimal,
    loc_growth_rate: Decimal,
) -> Element {
    let new_instances = produced() + clicks();
    let loc_cost = loc_base_cost * loc_growth_rate.pow(&new_instances);
    let disabled = loc() < loc_cost;
    rsx! {
        div {
            class: "repeatable-action",
            p {"{description}"}
            p {"Cost {loc_cost} loc"}
            button {
                disabled: disabled,
                class: "repeatable-action-button",
                onclick: move |_| {
                clicks += Decimal::new(1.0);
                logs.write().log(debug_message.as_str())
            }
            , {button_name} }
        }
    }
}
