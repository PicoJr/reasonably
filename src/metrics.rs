use std::collections::HashSet;
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;
use crate::simple_logs::SimpleLogs;
use crate::format_decimal::{format_decimal_loc, format_decimal_bugs};

#[component]
pub(crate) fn Metrics(
    mut logs: Signal<SimpleLogs>,
    researched: Signal<HashSet<String>>,
    loc_dt: Signal<Decimal>,
    bugs_dt: Signal<Decimal>,
    dt: Signal<Decimal>,
) -> Element {
    rsx! {
        div {
            class: "metrics",
            table {
                class: "metrics-table",
                tr {
                    th {"metrics"}
                    th {"value"}
                }
                tr {
                    td {"LOC/s"}
                    td {"{format_decimal_loc(loc_dt())}"}
                }
                tr {
                    td {"bugs/s"}
                    td {"{format_decimal_bugs(bugs_dt())}"}
                }
                if researched().contains("cheating") {
                    tr {
                        td {"dt"}
                        td {"{dt()}"}
                    }
                }
            }
        }
    }
}