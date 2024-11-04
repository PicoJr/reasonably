use std::collections::HashSet;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;

use crate::simple_logs::SimpleLogs;
use crate::Theme;

#[component]
pub(crate) fn ToggleThemeAction(
    mut logs: Signal<SimpleLogs>,
    mut researched: Signal<HashSet<String>>,
    mut theme: Signal<Theme>,
    require: Option<String>,
) -> Element {
    let current_theme: Theme = theme();
    let requirements_met = require.map_or_else(
        || true,
        |research_name_required| researched().contains(research_name_required.as_str())
    );
    rsx! {
        if requirements_met {
            button {
                class: "repeatable-action-button",
                onclick: move |_| {
                match current_theme {
                    Theme::LightTheme => {
                        *theme.write() = Theme::DarkTheme;
                        logs.write().log(
                            "toggling theme...now dark"
                        );
                        spawn(async move {
                            eval(r#"
                            document.documentElement.setAttribute('data-theme', "dark")
                            "#,
                            ).await.expect("failed to run JS");
                        });
                    },
                    Theme::DarkTheme => {
                        *theme.write() = Theme::LightTheme;
                        logs.write().log(
                            "toggling theme...now light"
                        );
                        spawn(async move {
                            eval(r#"
                            document.documentElement.setAttribute('data-theme', "light")
                            "#,
                            ).await.expect("failed to run JS");
                        });
                    },
                };
            }
            , "Toggle Theme" }
        }
    }
}
