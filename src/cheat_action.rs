use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;
use crate::simple_logs::SimpleLogs;

#[component]
pub(crate) fn CheatAction(
    mut logs: Signal<SimpleLogs>,
    mut value: Signal<Decimal>,
    button_name: String,
    debug_message: String,
) -> Element {
    rsx! {
        button {
            class: "repeatable-action-button",
            onclick: move |_| {
            value *= Decimal::new(2.0);
            logs.write().log(
                debug_message.as_str()
            )
        }
        , {button_name} }
    }
}
