#![allow(non_snake_case)]
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal};
use dioxus::prelude::*;
use crate::format_decimal::{format_decimal_loc, format_decimal_bugs, format_decimal_devs, format_decimal_hrs, format_decimal_pms, format_decimal_features};
use crate::state::State;

#[component]
pub(crate) fn Resources(
    state: Signal<State>,
) -> Element {
    rsx! {
        div {
            class: "resources",
            table {
                class: "resources-table",
                tr {
                    th {
                        class: "table-name",
                        "resources"
                    }
                    th {
                        class: "table-value",
                        "value"
                    }
                }
                if state.read().loc > Decimal::ZERO {
                    tr {
                        td {"Lines of code"}
                        td {
                            class: "table-value",
                            "{format_decimal_loc(state.read().loc)}"
                        }
                    }
                }
                if state.read().bugs > Decimal::ZERO {
                    tr {
                        td{"Bugs"}
                        td{
                            class: "table-value",
                            "{format_decimal_bugs(state.read().bugs)}"
                        }
                    }
                }
                if state.read().features > Decimal::ZERO {
                    tr {
                        td{"Features"}
                        td{
                            class: "table-value",
                            "{format_decimal_features(state.read().features)}"
                        }
                    }
                }
                if state.read().interns > Decimal::ZERO {
                    tr {
                        td {"Interns (auto)"}
                        td {
                            class: "table-value",
                            "{format_decimal_devs(state.read().interns)}"
                        }
                    }
                }
                if state.read().manual_interns > Decimal::ZERO {
                    tr {
                        td {"Interns"}
                        td {
                            class: "table-value",
                            "{format_decimal_devs(state.read().manual_interns)}"
                        }
                    }
                }
                if state.read().junior_devs > Decimal::ZERO {
                    tr {
                        td {"Junior devs (auto)"}
                        td {
                            class: "table-value",
                            "{format_decimal_devs(state.read().junior_devs)}"
                        }
                    }
                }
                if state.read().manual_junior_devs > Decimal::ZERO {
                    tr {
                        td {"Junior devs"}
                        td {
                            class: "table-value",
                            "{format_decimal_devs(state.read().manual_junior_devs)}"
                        }
                    }
                }
                if state.read().senior_devs > Decimal::ZERO {
                    tr {
                        td{"Senior devs (auto)"}
                        td{
                            class: "table-value",
                            "{format_decimal_devs(state.read().senior_devs)}"
                        }
                    }
                }
                if state.read().manual_senior_devs > Decimal::ZERO {
                    tr {
                        td{"Senior devs"}
                        td{
                            class: "table-value",
                            "{format_decimal_devs(state.read().manual_senior_devs)}"
                        }
                    }
                }
                if state.read().retired_devs > Decimal::ZERO {
                    tr {
                        td{"Retired devs"}
                        td{
                            class: "table-value",
                            "{format_decimal_devs(state.read().retired_devs)}"
                        }
                    }
                }
                if state.read().hrs > Decimal::ZERO {
                    tr {
                        td{"HRs (auto)"}
                        td{
                            class: "table-value",
                            "{format_decimal_hrs(state.read().hrs)}"
                        }
                    }
                }
                if state.read().manual_hrs > Decimal::ZERO {
                    tr {
                        td{"HRs"}
                        td{
                            class: "table-value",
                            "{format_decimal_hrs(state.read().manual_hrs)}"
                        }
                    }
                }
                if state.read().pms > Decimal::ZERO {
                    tr {
                        td{"PMs (auto)"}
                        td{
                            class: "table-value",
                            "{format_decimal_pms(state.read().pms)}"
                        }
                    }
                }
                if state.read().manual_pms > Decimal::ZERO {
                    tr {
                        td{"PMs"}
                        td{
                            class: "table-value",
                            "{format_decimal_pms(state.read().manual_pms)}"
                        }
                    }
                }
            }
        }
    }
}
