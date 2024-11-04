#![allow(non_snake_case)]
use std::collections::HashSet;
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal};
use dioxus::prelude::*;
use crate::format_decimal::{format_decimal_loc, format_decimal_bugs};

#[component]
pub(crate) fn Metrics(
    researched: Signal<HashSet<String>>,
    loc_dt: Signal<Decimal>,
    bugs_dt: Signal<Decimal>,
    dt: Signal<Decimal>,
) -> Element {
    rsx! {
        if researched().contains("code_metrics") {
            div {
                class: "metrics",
                table {
                    class: "metrics-table",
                    tr {
                        th {
                            class: "table-name",
                            "metrics"
                        }
                        th {
                            class: "table-value",
                            "value"
                        }
                    }
                    tr {
                        td {"LOC/s"}
                        td {
                            class: "table-value",
                            "{format_decimal_loc(loc_dt())}"
                        }
                    }
                    tr {
                        td {"bugs/s"}
                        td {
                            class: "table-value",
                            "{format_decimal_bugs(bugs_dt())}"
                        }
                    }
                    if researched().contains("cheating") {
                        tr {
                            td {"dt"}
                            td {
                                class: "table-value",
                                "{dt()}"
                            }
                        }
                    }
                }
            }
        }
    }
}