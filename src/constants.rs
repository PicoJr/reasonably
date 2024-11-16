use break_infinity::Decimal;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GameConstants {
    // interns recruitment cost
    pub interns_loc_base_cost: Decimal,
    pub interns_loc_growth_rate: Decimal,
    // junior devs recruitment cost
    pub junior_devs_loc_base_cost: Decimal,
    pub junior_devs_loc_growth_rate: Decimal,
    // senior devs recruitment cost
    pub senior_devs_loc_base_cost: Decimal,
    pub senior_devs_loc_growth_rate: Decimal,
    // hr recruitment cost
    pub hrs_loc_base_cost: Decimal,
    pub hrs_loc_growth_rate: Decimal,
    // pm recruitment cost
    pub pms_loc_base_cost: Decimal,
    pub pms_loc_growth_rate: Decimal,
    // research once costs
    pub research_internship_loc_cost: Decimal,
    pub research_junior_devs_position_loc_cost: Decimal,
    pub research_senior_devs_position_loc_cost: Decimal,
    pub research_human_resources_loc_cost: Decimal,
    pub research_project_management_loc_cost: Decimal,
    pub research_interns_promotion_loc_cost: Decimal,
    pub research_junior_devs_promotion_loc_cost: Decimal,
    pub research_code_metrics_loc_cost: Decimal,
    pub research_speedrun_loc_cost: Decimal,
    pub research_logs_loc_cost: Decimal,
    pub research_rmrf_loc_cost: Decimal,
    pub research_toggle_theme_loc_cost: Decimal,
    pub research_syntax_coloring_multiplier_loc_cost: Decimal,
    pub research_management_career_loc_cost: Decimal,
    pub research_smart_staffing_loc_cost: Decimal,
    pub research_recursive_hr_loc_cost: Decimal,
    // initial promotion ratio
    pub interns_promotion_ratio_dt: Decimal,
    pub junior_devs_promotion_ratio_dt: Decimal,
    pub senior_devs_retirement_ratio_dt: Decimal,
    // devs loc production
    pub interns_loc_dt: Decimal,
    pub junior_devs_loc_dt: Decimal,
    pub senior_devs_loc_dt: Decimal,
    // hr recruitment
    pub hrs_interns_dt: Decimal,
    pub hrs_interns_quota: Decimal,
    pub hrs_junior_devs_dt: Decimal,
    pub hrs_junior_devs_quota: Decimal,
    pub hrs_senior_devs_dt: Decimal,
    pub hrs_senior_devs_quota: Decimal,
    pub hrs_hrs_dt: Decimal,
    pub hrs_hrs_quota: Decimal,
    // pm bugs conversion
    pub pms_bugs_conversion_dt: Decimal,
    // manual loc production
    pub loc_per_clicks: Decimal,
    pub debug_per_clicks: Decimal,
    // bugs ratio
    pub manual_bugs_ratio: Decimal,
    pub interns_bugs_ratio: Decimal,
    pub junior_devs_bugs_ratio: Decimal,
    pub senior_devs_bugs_ratio: Decimal,
    // quests
    pub quest_hello_world_loc_cost: Decimal,
    pub quest_fizz_buzz_loc_cost: Decimal,
    pub quest_calculator_loc_cost: Decimal,
    pub quest_game_of_life_loc_cost: Decimal,
    pub quest_text_editor_loc_cost: Decimal,
    pub quest_physics_engine_loc_cost: Decimal,
    pub quest_bacteria_loc_cost: Decimal,
    pub quest_browser_loc_cost: Decimal,
    pub quest_kernel_loc_cost: Decimal,
    pub quest_mouse_loc_cost: Decimal,
    pub quest_human_brain_loc_cost: Decimal,
    pub quest_economy_loc_cost: Decimal,
    pub quest_climate_loc_cost: Decimal,
    pub quest_earth_loc_cost: Decimal,
    pub quest_solar_system_loc_cost: Decimal,
    pub quest_universe_loc_cost: Decimal,
    pub quest_differentiation_loc_cost: Decimal,
    // multipliers
    pub research_syntax_coloring_multiplier: Decimal,
    pub senior_devs_management_career_ratio: Decimal,
    pub dt: Decimal,
}

impl Default for GameConstants {
    fn default() -> Self {
        GameConstants {
            interns_loc_base_cost: Decimal::new(20.0),
            interns_loc_growth_rate: Decimal::new(1.015),
            junior_devs_loc_base_cost: Decimal::new(1250.0),
            junior_devs_loc_growth_rate: Decimal::new(1.02),
            senior_devs_loc_base_cost: Decimal::new(15_000.0),
            senior_devs_loc_growth_rate: Decimal::new(1.0175),
            hrs_loc_base_cost: Decimal::new(15_000.0),
            hrs_loc_growth_rate: Decimal::new(1.0175),
            pms_loc_base_cost: Decimal::new(15_000.0),
            pms_loc_growth_rate: Decimal::new(1.0175),
            research_internship_loc_cost: Decimal::new(1.0),
            research_junior_devs_position_loc_cost: Decimal::new(1.0),
            research_senior_devs_position_loc_cost: Decimal::new(1.0),
            research_human_resources_loc_cost: Decimal::new(1.0),
            research_project_management_loc_cost: Decimal::new(1.0),
            research_interns_promotion_loc_cost: Decimal::new(1.0),
            research_junior_devs_promotion_loc_cost: Decimal::new(1.0),
            research_code_metrics_loc_cost: Decimal::new(1.0),
            research_speedrun_loc_cost: Decimal::new(1.0),
            research_logs_loc_cost: Decimal::new(1.0),
            research_rmrf_loc_cost: Decimal::new(1.0),
            research_toggle_theme_loc_cost: Decimal::new(1.0),
            research_syntax_coloring_multiplier_loc_cost: Decimal::new(1.0),
            research_management_career_loc_cost: Decimal::new(1.0),
            research_smart_staffing_loc_cost: Decimal::new(1.0),
            research_recursive_hr_loc_cost: Decimal::new(1.0),
            interns_promotion_ratio_dt: Decimal::new(0.04),
            junior_devs_promotion_ratio_dt: Decimal::new(0.02),
            senior_devs_retirement_ratio_dt: Decimal::new(0.01),
            interns_loc_dt: Decimal::new(1.0),
            junior_devs_loc_dt: Decimal::new(40.0),
            senior_devs_loc_dt: Decimal::new(1000.0),
            hrs_interns_dt: Decimal::new(1e-2),
            hrs_interns_quota: Decimal::new(0.90),
            hrs_junior_devs_dt: Decimal::new(1e-3),
            hrs_junior_devs_quota: Decimal::new(0.09),
            hrs_senior_devs_dt: Decimal::new(1e-4),
            hrs_senior_devs_quota: Decimal::new(0.01),
            hrs_hrs_dt: Decimal::new(1e-4),
            hrs_hrs_quota: Decimal::new(0.01),
            pms_bugs_conversion_dt: Decimal::new(1.0),
            loc_per_clicks: Decimal::new(1.0),
            debug_per_clicks: Decimal::new(1.0),
            manual_bugs_ratio: Decimal::new(1.0),
            interns_bugs_ratio: Decimal::new(2.0),
            junior_devs_bugs_ratio: Decimal::new(1.5),
            senior_devs_bugs_ratio: Decimal::new(1.0),
            quest_hello_world_loc_cost: Decimal::new(1.0),
            quest_fizz_buzz_loc_cost: Decimal::new(10.0),
            quest_calculator_loc_cost: Decimal::new(100.0),
            quest_game_of_life_loc_cost: Decimal::new(1e3),
            quest_text_editor_loc_cost: Decimal::new(1e4),
            quest_physics_engine_loc_cost: Decimal::new(1e5),
            quest_bacteria_loc_cost: Decimal::new(1e6),
            quest_browser_loc_cost: Decimal::new(5e6),
            quest_kernel_loc_cost: Decimal::new(30e6),
            quest_mouse_loc_cost: Decimal::new(100e6),
            quest_human_brain_loc_cost: Decimal::new(1e9),
            quest_economy_loc_cost: Decimal::new(10e9),
            quest_climate_loc_cost: Decimal::new(100e9),
            quest_earth_loc_cost: Decimal::new(1e12),
            quest_solar_system_loc_cost: Decimal::new(1e15),
            quest_universe_loc_cost: Decimal::new(1e18),
            quest_differentiation_loc_cost: Decimal::new(1e21),
            research_syntax_coloring_multiplier: Decimal::new(2.0),
            senior_devs_management_career_ratio: Decimal::new(0.5),
            dt: Decimal::new(0.01),
        }
    }
}

/**
 *For optimization purpose: avoid using String when all research names are known ahead of time
 */
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[repr(u32)]
pub(crate) enum Research {
    Bacteria,
    Browser,
    Calculator,
    Cheating,
    Climate,
    CodeMetrics,
    Differentiation,
    Earth,
    Economy,
    FizzBuzz,
    GameOfLife,
    HelloWorld,
    HumanBrain,
    HumanResources,
    Internship,
    InternsPromotion,
    JuniorDevsPosition,
    JuniorDevsPromotion,
    Kernel,
    Logs,
    ManagementCareer,
    Mouse,
    PhysicsEngine,
    ProjectManagement,
    RecursiveHR,
    Rmrf,
    SeniorDevsPosition,
    SmartStaffing,
    SolarSystem,
    Speedrun,
    SyntaxColoringMultiplier,
    TextEditor,
    ToggleTheme,
    Universe,
}
