use break_infinity::Decimal;

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
    // research once costs
    pub research_internship_loc_cost: Decimal,
    pub research_interns_promotion_loc_cost: Decimal,
    pub research_junior_devs_promotion_loc_cost: Decimal,
    pub research_code_metrics_loc_cost: Decimal,
    pub research_speedrun_loc_cost: Decimal,
    pub research_logs_loc_cost: Decimal,
    pub research_rmrf_loc_cost: Decimal,
    pub research_toggle_theme_loc_cost: Decimal,
    // initial promotion ratio
    pub interns_promotion_ratio_dt: Decimal,
    pub junior_devs_promotion_ratio_dt: Decimal,
    pub senior_devs_retirement_ratio_dt: Decimal,
    // devs loc production
    pub interns_loc_dt: Decimal,
    pub junior_devs_loc_dt: Decimal,
    pub senior_devs_loc_dt: Decimal,
    // manual loc production
    pub loc_per_clicks: Decimal,
    pub debug_per_clicks: Decimal,
    // bugs ratio
    pub manual_bugs_ratio: Decimal,
    pub interns_bugs_ratio: Decimal,
    pub junior_devs_bugs_ratio: Decimal,
    pub senior_devs_bugs_ratio: Decimal,
    // max loc for progress tracking
    pub max_loc: Decimal,
    // quests
    pub quest_hello_world_loc_cost: Decimal,
}

impl Default for GameConstants {
    fn default() -> Self {
        GameConstants {
            interns_loc_base_cost: Decimal::new(10.0),
            interns_loc_growth_rate: Decimal::new(1.1),
            junior_devs_loc_base_cost: Decimal::new(20.0),
            junior_devs_loc_growth_rate: Decimal::new(1.1),
            senior_devs_loc_base_cost: Decimal::new(40.0),
            senior_devs_loc_growth_rate: Decimal::new(1.1),
            research_internship_loc_cost: Decimal::new(1.0),
            research_interns_promotion_loc_cost: Decimal::new(1.0),
            research_junior_devs_promotion_loc_cost: Decimal::new(1.0),
            research_code_metrics_loc_cost: Decimal::new(1.0),
            research_speedrun_loc_cost: Decimal::new(1.0),
            research_logs_loc_cost: Decimal::new(1.0),
            research_rmrf_loc_cost: Decimal::new(1.0),
            research_toggle_theme_loc_cost: Decimal::new(1.0),
            interns_promotion_ratio_dt: Decimal::new(0.04),
            junior_devs_promotion_ratio_dt: Decimal::new(0.02),
            senior_devs_retirement_ratio_dt: Decimal::new(0.01),
            interns_loc_dt: Decimal::new(1.0),
            junior_devs_loc_dt: Decimal::new(1.5),
            senior_devs_loc_dt: Decimal::new(2.0),
            loc_per_clicks: Decimal::new(1.0),
            debug_per_clicks: Decimal::new(1.0),
            manual_bugs_ratio: Decimal::new(1.0),
            interns_bugs_ratio: Decimal::new(2.0),
            junior_devs_bugs_ratio: Decimal::new(1.5),
            senior_devs_bugs_ratio: Decimal::new(1.0),
            max_loc: Decimal::new(1e21),
            quest_hello_world_loc_cost: Decimal::new(1.0),
        }
    }
}