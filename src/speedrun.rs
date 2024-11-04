#![allow(non_snake_case)]
use std::collections::HashSet;
use web_time::{Instant};

use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal};
use dioxus::prelude::*;

#[component]
pub(crate) fn Speedrun(
    researched: Signal<HashSet<String>>,
    loc: Signal<Decimal>,
    speedrun_start: Signal<Option<Instant>>,
    current_time: Signal<Instant>,
    max_loc: Decimal,
) -> Element {
    let progress = (loc().max(&Decimal::ONE).log10() / max_loc.max(&Decimal::ONE).log10()).clamp(0.0, 1.0);
    let elapsed_time = if let Some(start) = speedrun_start() {
        let duration = current_time() - start;
        let millis = duration.as_millis();
        let hundredth = (millis / 10u128) % 100u128;
        let seconds = (millis / 1000u128) % 60u128;
        let minutes = (millis / 60_000u128) % 60u128;
        let hours = millis / 3600_000u128;
        format!("{}:{:02}:{:02}.{:02}", hours, minutes, seconds, hundredth)
    }  else {
        "timer not started".to_string()
    };
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
                                value: progress,
                                max: 1.0,
                            }
                        }
                        td {
                            class: "table-value",
                            "{elapsed_time}"
                        }
                    }
                }
            }
        }
    }
}
