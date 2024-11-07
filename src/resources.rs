#![allow(non_snake_case)]
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal};
use dioxus::prelude::*;
use crate::format_decimal::{format_decimal_loc, format_decimal_bugs, format_decimal_devs, format_decimal_hrs};

#[component]
pub(crate) fn Resources(
    loc: Signal<Decimal>,
    bugs: Signal<Decimal>,
    interns: Signal<Decimal>,
    junior_devs: Signal<Decimal>,
    senior_devs: Signal<Decimal>,
    retired_devs: Signal<Decimal>,
    hrs: Signal<Decimal>,
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
                if loc() > Decimal::ZERO {
                    tr {
                        td {"Lines of code"}
                        td {
                            class: "table-value",
                            "{format_decimal_loc(loc())}"
                        }
                    }
                }
                if bugs() > Decimal::ZERO {
                    tr {
                        td{"Bugs"}
                        td{
                            class: "table-value",
                            "{format_decimal_bugs(bugs())}"
                        }
                    }
                }
                if interns() > Decimal::ZERO {
                    tr {
                        td {"Interns"}
                        td {
                            class: "table-value",
                            "{format_decimal_devs(interns())}"
                        }
                    }
                }
                if junior_devs() > Decimal::ZERO {
                    tr {
                        td {"Junior devs"}
                        td {
                            class: "table-value",
                            "{format_decimal_devs(junior_devs())}"
                        }
                    }
                }
                if senior_devs() > Decimal::ZERO {
                    tr {
                        td{"Senior devs"}
                        td{
                            class: "table-value",
                            "{format_decimal_devs(senior_devs())}"
                        }
                    }
                }
                if retired_devs() > Decimal::ZERO {
                    tr {
                        td{"Retired devs"}
                        td{
                            class: "table-value",
                            "{format_decimal_devs(retired_devs())}"
                        }
                    }
                }
                if hrs() > Decimal::ZERO {
                    tr {
                        td{"HRs"}
                        td{
                            class: "table-value",
                            "{format_decimal_hrs(hrs())}"
                        }
                    }
                }
            }
        }
    }
}
