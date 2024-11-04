#![allow(non_snake_case)]
use std::collections::HashSet;
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;
use crate::simple_logs::SimpleLogs;
use crate::format_decimal::{format_decimal_loc, format_decimal_bugs};

#[component]
pub(crate) fn Speedrun(
    researched: Signal<HashSet<String>>,
    loc: Signal<Decimal>,
) -> Element {
    rsx! {
        if researched().contains("speedrun") {
            div {
                class: "speedrun",
                table {
                    class: "speedrun-table",
                    tr {
                        th {
                            class: "table-name",
                            "progress"
                        }
                        th {
                            class: "table-value",
                            "time"
                        }
                    }
                    tr {
                        td {
                            progress {
                                value: 20.0,
                                max: 100.0,
                            }
                        }
                        td {
                            class: "table-value",
                            "{format_decimal_loc(loc())}"
                        }
                    }
                }
            }
        }
    }
}
