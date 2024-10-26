#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn DebugAction(mut bug_count: Signal<i32>) -> Element {
    rsx! {
        button { onclick: move |_| bug_count -= 1, "Debug" }
    }
}

#[component]
fn CodeAction(
    mut loc_count: Signal<i32>,
    mut bug_count: Signal<i32>,
    loc_increment: Signal<i32>,
    bug_loc_ratio: Signal<f32>,
) -> Element {
    rsx! {
        button { onclick: move |_| {
            loc_count += loc_increment();
            bug_count += (loc_increment() as f32 * bug_loc_ratio()) as i32;
        }
        , "Code" }
    }
}

#[component]
fn Home() -> Element {
    let mut loc_count: Signal<i32> = use_signal(|| 0);
    let mut loc_increment: Signal<i32> = use_signal(|| 1);
    let bug_count: Signal<i32> = use_signal(|| 0);
    let bug_loc_ratio: Signal<f32> = use_signal(|| 2.0);

    rsx! {
        div {
            if loc_count() > 0 {
                h1 {"Lines of code {loc_count}"}
            }
            if bug_count() > 0 {
                h1 {"Bugs {bug_count}"}
            }
            CodeAction{loc_count, bug_count, loc_increment, bug_loc_ratio}
            if bug_count() > 0 {
                DebugAction {bug_count}
            }
            if loc_count() >= 20 && bug_count() <= 0 {
                button {onclick: move |_| {
                    loc_count -= 20;
                }, "Write a hello world"}
            }

            if loc_count() >= 10 && bug_count() <= 0 {
                button {onclick: move |_| {
                    loc_count -= 10;
                    loc_increment += 1;
                }, "Learn VIM"}
            }
        }
    }
}
