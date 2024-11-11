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

    pub(crate) fn update(&mut self, dt_seconds: Decimal) {
        // loc produced by devs
        let auto_loc = (
            (self.interns + self.manual_interns) * self.interns_loc_dt
                + (self.junior_devs + self.manual_junior_devs) * self.junior_devs_loc_dt
                + (self.senior_devs + self.manual_senior_devs) * self.senior_devs_loc_dt
        ) * self.dt;
        // bugs produced by devs
        let auto_bugs = (
            (self.interns + self.manual_interns) * self.interns_loc_dt * self.interns_bugs_ratio
                + (self.junior_devs + self.manual_junior_devs) * self.junior_devs_loc_dt * self.junior_devs_bugs_ratio
                + (self.senior_devs + self.manual_senior_devs) * self.senior_devs_loc_dt * self.senior_devs_bugs_ratio
        ) * self.dt;

        // update loc, accounting all sources
        self.loc += auto_loc;
        // update live code metrics
        self.loc_dt = auto_loc * dt_seconds;

        let auto_bugs_converted_capacity = (self.pms + self.manual_pms) * self.pms_bugs_conversion_dt * self.dt;
        // make sure we do not convert more bugs than available
        let bugs_converted = self.bugs.min(&auto_bugs_converted_capacity);
        let bugs_delta = auto_bugs - bugs_converted;
        self.bugs += bugs_delta;
        self.bugs_dt = bugs_delta * dt_seconds;

        self.features += bugs_converted;
        self.features_dt = bugs_converted * dt_seconds;

        let auto_interns = (self.hrs + self.manual_hrs) * self.hrs_interns_dt * self.hrs_interns_quota * self.dt;
        let auto_junior_devs = (self.hrs + self.manual_hrs) * self.hrs_junior_devs_dt * self.hrs_junior_devs_quota * self.dt;
        let auto_senior_devs = (self.hrs + self.manual_hrs) * self.hrs_senior_devs_dt * self.hrs_senior_devs_quota * self.dt;

        // update interns, junior devs, senior devs count, accounting for all sources
        self.interns += auto_interns;
        self.junior_devs += auto_junior_devs;
        self.senior_devs += auto_senior_devs;
        let seniors_becoming_pms = (self.senior_devs + self.manual_senior_devs) * self.senior_devs_management_ratio_dt * self.dt;
        self.pms += seniors_becoming_pms;

        // handle promotions & retirement...
        let retired_seniors = (self.senior_devs + self.manual_senior_devs) * self.senior_devs_retirement_ratio_dt * self.dt;
        self.retired_devs += retired_seniors;
        let remaining_seniors = self.senior_devs * (Decimal::ONE - self.senior_devs_retirement_ratio_dt * self.dt);
        self.senior_devs = remaining_seniors;
        let remaining_manual_seniors =self.manual_senior_devs * (Decimal::ONE - self.senior_devs_retirement_ratio_dt * self.dt);
        self.manual_senior_devs = remaining_manual_seniors;

        if self.researched.contains(&Research::JuniorDevsPromotion) {
            let juniors_promoted_to_seniors = (self.junior_devs + self.manual_junior_devs) * self.junior_devs_promotion_ratio_dt * self.dt;
            self.senior_devs += juniors_promoted_to_seniors;
            let remaining_juniors = self.junior_devs * (Decimal::ONE - self.junior_devs_promotion_ratio_dt * self.dt);
            self.junior_devs = remaining_juniors;
            let remaining_manual_juniors = self.manual_junior_devs * (Decimal::ONE - self.junior_devs_promotion_ratio_dt * self.dt);
            self.manual_junior_devs = remaining_manual_juniors;
        }

        if self.researched.contains(&Research::InternsPromotion) {
            let interns_promoted_juniors = (self.interns + self.manual_interns) * self.interns_promotion_ratio_dt * self.dt;
            self.junior_devs += interns_promoted_juniors;
            let remaining_interns = self.interns * (Decimal::ONE - self.interns_promotion_ratio_dt * self.dt);
            self.interns = remaining_interns;
            let remaining_manual_interns = self.manual_interns * (Decimal::ONE - self.interns_promotion_ratio_dt * self.dt);
            self.manual_interns = remaining_manual_interns;
        }

        // update current time
        self.current_time = Instant::now();
    }
}

impl Default for State {
    fn default() -> Self {
        let constants = GameConstants::default();
        Self::new(constants)
    }
}