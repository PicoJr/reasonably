#![allow(non_snake_case)]
use crate::constants::Research;
use crate::format_decimal::{format_decimal_bugs, format_decimal_features, format_decimal_loc};
use crate::state::State;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::Signal;
use dioxus::prelude::*;

#[component]
pub(crate) fn Metrics(state: Signal<State>) -> Element {
    rsx! {
        if state.read().researched.contains(&Research::CodeMetrics) {
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
                            "{format_decimal_loc(state.read().loc_dt)}"
                        }
                    }
                    tr {
                        td {"bugs/s"}
                        td {
                            class: "table-value",
                            "{format_decimal_bugs(state.read().bugs_dt)}"
                        }
                    }
                    tr {
                        td {"feature/s"}
                        td {
                            class: "table-value",
                            "{format_decimal_features(state.read().features_dt)}"
                        }
                    }
                    if state.read().researched.contains(&Research::Cheating) {
                        tr {
                            td {"dt"}
                            td {
                                class: "table-value",
                                "{state.read().dt}"
                            }
                        }
                    }
                }
            }
        }
    }
}
