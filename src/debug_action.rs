#![allow(non_snake_case)]
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;
use crate::simple_logs::SimpleLogs;

#[component]
pub(crate) fn DebugAction(mut logs: Signal<SimpleLogs>, mut debug_clicks: Signal<Decimal>) -> Element {
    rsx! {
        button {
            class: "repeatable-action-button",
            onclick: move |_| {
            debug_clicks += Decimal::new(1.0);
            logs.write().log(
                "debugging..."
            )
        }
        , "Debug" }
    }
}
