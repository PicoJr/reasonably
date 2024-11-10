use std::collections::HashSet;
use break_infinity::Decimal;
use web_time::Instant;
use crate::constants::{GameConstants, Research};
use crate::simple_logs::SimpleLogs;
use crate::Theme;

pub(crate) struct State {
    pub logs: SimpleLogs,
    pub researched: HashSet<Research>,
    pub theme: Theme,
    pub speedrun_start: Option<Instant>,
    pub current_time: Instant,
    pub loc_dt: Decimal,
    pub bugs_dt: Decimal,
    pub features_dt: Decimal,
    pub loc_per_clicks: Decimal,
    pub debug_per_clicks: Decimal,
    pub manual_bugs_ratio: Decimal,
    pub interns_bugs_ratio: Decimal,
    pub junior_devs_bugs_ratio: Decimal,
    pub senior_devs_bugs_ratio: Decimal,
    pub loc: Decimal,
    pub bugs: Decimal,
    pub features: Decimal,
    pub manual_interns: Decimal,
    pub manual_junior_devs: Decimal,
    pub manual_senior_devs: Decimal,
    pub manual_hrs: Decimal,
    pub manual_pms: Decimal,
    pub interns: Decimal,
    pub interns_loc_dt: Decimal,
    pub junior_devs: Decimal,
    pub junior_devs_loc_dt: Decimal,
    pub senior_devs: Decimal,
    pub senior_devs_loc_dt: Decimal,
    pub retired_devs: Decimal,
    pub hrs: Decimal,
    pub hrs_interns_dt: Decimal,
    pub hrs_interns_quota: Decimal,
    pub hrs_junior_devs_dt: Decimal,
    pub hrs_junior_devs_quota: Decimal,
    pub hrs_senior_devs_dt: Decimal,
    pub hrs_senior_devs_quota: Decimal,
    pub pms: Decimal,
    pub pms_bugs_conversion_dt: Decimal,
    pub interns_promotion_ratio_dt: Decimal,
    pub junior_devs_promotion_ratio_dt: Decimal,
    pub senior_devs_retirement_ratio_dt: Decimal,
    pub senior_devs_management_ratio_dt: Decimal,
    pub dt: Decimal,
}

impl State {
    pub(crate) fn new(constants: GameConstants) -> State {
        let researched = HashSet::from([Research::Cheating]);
        State {
            logs: SimpleLogs::new(),
            researched,
            theme: Theme::LightTheme,
            speedrun_start: None,
            current_time: Instant::now(),
            loc_dt: Default::default(),
            bugs_dt: Default::default(),
            features_dt: Default::default(),
            loc_per_clicks: constants.loc_per_clicks,
            debug_per_clicks: constants.debug_per_clicks,
            manual_bugs_ratio: constants.manual_bugs_ratio,
            interns_bugs_ratio: constants.interns_bugs_ratio,
            junior_devs_bugs_ratio: constants.junior_devs_bugs_ratio,
            senior_devs_bugs_ratio: constants.senior_devs_bugs_ratio,
            loc: Default::default(),
            bugs: Default::default(),
            features: Default::default(),
            manual_interns: Default::default(),
            manual_junior_devs: Default::default(),
            manual_senior_devs: Default::default(),
            manual_hrs: Default::default(),
            manual_pms: Default::default(),
            interns: Default::default(),
            interns_loc_dt: constants.interns_loc_dt,
            junior_devs: Default::default(),
            junior_devs_loc_dt: constants.junior_devs_loc_dt,
            senior_devs: Default::default(),
            senior_devs_loc_dt: constants.senior_devs_loc_dt,
            retired_devs: Default::default(),
            hrs: Default::default(),
            hrs_interns_dt: constants.hrs_interns_dt,
            hrs_interns_quota: constants.hrs_interns_quota,
            hrs_junior_devs_dt: constants.hrs_junior_devs_dt,
            hrs_junior_devs_quota: constants.hrs_junior_devs_quota,
            hrs_senior_devs_dt: constants.hrs_senior_devs_dt,
            hrs_senior_devs_quota: constants.hrs_senior_devs_quota,
            pms: Default::default(),
            pms_bugs_conversion_dt: constants.pms_bugs_conversion_dt,
            interns_promotion_ratio_dt: constants.interns_promotion_ratio_dt,
            junior_devs_promotion_ratio_dt: constants.junior_devs_promotion_ratio_dt,
            senior_devs_retirement_ratio_dt: constants.senior_devs_retirement_ratio_dt,
            senior_devs_management_ratio_dt: Default::default(),
            dt: constants.dt,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let constants = GameConstants::default();
        Self::new(constants)
    }
}