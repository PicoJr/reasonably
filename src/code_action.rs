#![allow(non_snake_case)]
use web_time::{Instant};
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::Signal;
use dioxus::prelude::*;
use crate::simple_logs::SimpleLogs;

#[component]
pub(crate) fn CodeAction(
    mut logs: Signal<SimpleLogs>,
    mut code_clicks: Signal<Decimal>,
    mut speedrun_start: Signal<Option<Instant>>,
) -> Element {
    rsx! {
        button {
            class: "repeatable-action-button",
            onclick: move |_| {
                code_clicks += Decimal::new(1.0);
                logs.write().log("coding...");
                if speedrun_start().is_none() {
                    *speedrun_start.write() = Some(Instant::now());
                }
        }
        , "Code" }
    }
}
