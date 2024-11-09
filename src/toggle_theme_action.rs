#![allow(non_snake_case)]
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::{Signal, Writable};
use dioxus::prelude::*;
use crate::constants::Research;

use crate::state::State;
use crate::Theme;

#[component]
pub(crate) fn ToggleThemeAction(
    mut state: Signal<State>,
) -> Element {
    let current_theme: Theme = state.read().theme.clone();
    rsx! {
        if state.read().researched.contains(&Research::ToggleTheme) {
            button {
                class: "repeatable-action-button",
                onclick: move |_| {
                match current_theme {
                    Theme::LightTheme => {
                        state.write().theme = Theme::DarkTheme;
                        state.write().logs.log(
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
                        state.write().theme = Theme::LightTheme;
                        state.write().logs.log(
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
