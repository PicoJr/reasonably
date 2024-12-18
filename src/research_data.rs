#![allow(non_snake_case)]
use crate::constants::{GameConstants, Research};
use break_infinity::Decimal;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use dioxus::prelude::{Signal, Writable};

use crate::research_once::ResearchOnce;
use crate::state::State;

#[component]
pub(crate) fn Researches(mut state: Signal<State>, constants: GameConstants) -> Element {
    rsx! {
        div { // vertical
            class: "researches",
            ResearchOnce{
                state: state,
                require: None,
                research_name: Research::ToggleTheme,
                button_name: "Install theme",
                description: "allow toggling theme",
                loc_cost: constants.research_toggle_theme_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: None,
                research_name: Research::Internship,
                button_name: "Research internship",
                description: "allow hiring interns, who produce loc and bugs automatically",
                loc_cost: constants.research_internship_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::Internship),
                research_name: Research::JuniorDevsPosition,
                button_name: "Research junior devs",
                description: "allow hiring junior devs, who produce loc and bugs automatically",
                loc_cost: constants.research_junior_devs_position_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::JuniorDevsPosition),
                research_name: Research::SeniorDevsPosition,
                button_name: "research senior devs",
                description: "Allow hiring senior devs, who produce loc and bugs automatically",
                loc_cost: constants.research_senior_devs_position_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: None,
                research_name: Research::CodeMetrics,
                button_name: "research code metrics",
                description: "Display LOC/s and bugs/s",
                loc_cost: constants.research_code_metrics_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: None,
                research_name: Research::Speedrun,
                button_name: "research speedrun",
                description: "Display progress bar and real time timer",
                loc_cost: constants.research_speedrun_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: None,
                research_name: Research::Logs,
                button_name: "research logs",
                description: "Display logs",
                loc_cost: constants.research_logs_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: None,
                research_name: Research::Rmrf,
                button_name: "learn rm -rf",
                description: "For desperate situations, allow using rm -rf command",
                loc_cost: constants.research_rmrf_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::JuniorDevsPosition),
                research_name: Research::InternsPromotion,
                button_name: "promote interns",
                description: "Allow interns to be promoted to junior devs",
                loc_cost: constants.research_interns_promotion_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::SeniorDevsPosition),
                research_name: Research::JuniorDevsPromotion,
                button_name: "promote junior devs",
                description: "Allow junior devs to be promoted to senior devs",
                loc_cost: constants.research_junior_devs_promotion_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::Internship),
                research_name: Research::SyntaxColoringMultiplier,
                button_name: "install syntax coloring",
                description: "Boost interns locs/s x2",
                loc_cost: constants.research_syntax_coloring_multiplier_loc_cost,
                quest: false,
                action: move |mut s: Signal<State>| {
                    s.write().interns_loc_dt *= constants.research_syntax_coloring_multiplier;
                },
            }
            ResearchOnce{
                state: state,
                require: Some(Research::SeniorDevsPosition),
                research_name: Research::HumanResources,
                button_name: "research human resources",
                description: "Allow hiring HR, who hire devs",
                loc_cost: constants.research_human_resources_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::SeniorDevsPosition),
                research_name: Research::ProjectManagement,
                button_name: "research project management",
                description: "Allow hiring PM, who convert bugs to features",
                loc_cost: constants.research_project_management_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::HumanResources),
                research_name: Research::SmartStaffing,
                button_name: "research smart staffing",
                description: "Smart staffing a.k.a massive layoffs",
                loc_cost: constants.research_smart_staffing_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::HumanResources),
                research_name: Research::RecursiveHR,
                button_name: "research recursive HR",
                description: "HR now recruit themselves",
                loc_cost: constants.research_recursive_hr_loc_cost,
                quest: false,
            }
            ResearchOnce{
                state: state,
                require: Some(Research::ProjectManagement),
                research_name: Research::ManagementCareer,
                button_name: "research management career",
                description: "Instead of retiring, some senior devs will become PMs",
                loc_cost: constants.research_management_career_loc_cost,
                quest: false,
                action: move |mut s: Signal<State>| {
                    let retirement_ratio_dt = s.read().senior_devs_retirement_ratio_dt;
                    s.write().senior_devs_retirement_ratio_dt = retirement_ratio_dt * (Decimal::ONE - constants.senior_devs_management_career_ratio);
                    s.write().senior_devs_management_ratio_dt = retirement_ratio_dt * constants.senior_devs_management_career_ratio;
                },
            }
        }
    }
}
