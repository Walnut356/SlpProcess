pub mod combos;
pub mod defense;
pub mod inputs;
pub mod items;
pub mod lcancel;
pub mod wavedash;
pub mod tech;

use std::ops::Div;

use polars::prelude::*;

use strum_macros::{EnumString, IntoStaticStr};

#[derive(Debug, Default, Clone)]
pub struct Stats {
    /// Minimum Replay Version: Any
    pub input: DataFrame,
    /// Minimum Replay Version: Any
    pub wavedash: DataFrame,
    /// Minimum Replay Version: 2.0.0
    pub l_cancel: Option<DataFrame>,
    /// Minimum Replay Version: 2.0.0
    pub tech: Option<DataFrame>,
    /// Minimum Replay Version: 3.0.0
    pub item: Option<DataFrame>,
    /// Minimum Replay Version 3.5.0
    pub defense: Option<DataFrame>,
}

impl Stats {
    /// Returns an owned stat object
    pub fn get(&self, stat_type: StatType) -> Option<DataFrame> {
        match stat_type {
            StatType::Input => Some(self.input.clone()),
            StatType::Wavedash => Some(self.wavedash.clone()),
            StatType::LCancel => self.l_cancel.clone(),
            StatType::Tech => self.tech.clone(),
            StatType::Item => self.item.clone(),
            StatType::Defense => self.defense.clone(),
        }
    }

    /// Returns a complete summary of all stats
    pub fn summarize(&self) -> DataFrame {
        todo!()
    }

    /// Returns a summary of
    pub fn get_summary(&self, stat_type: StatType) -> Option<DataFrame> {
        match stat_type {
            // These are already basically summaries anyway
            StatType::Input => Some(self.input.clone()),
            StatType::Item => self.item.clone(),
            StatType::Wavedash => self.wd_summary(),
            StatType::LCancel => self.lc_summary(),
            StatType::Tech => Some(DataFrame::default()),
            StatType::Defense => self.def_summary(),
        }
    }

    // And below is where we learn why I don't really like polars' Rust api. Extracting individual
    // values is incredibly verbose even if you know exactly what you want. The price we pay for
    // AnyValue I suppose

    fn wd_summary(&self) -> Option<DataFrame> {
        use crate::columns::WavedashStats as clm;
        let lf = self.wavedash.clone().lazy();

        lf.select(&[
            col(clm::Waveland.into())
                .filter(col(clm::Waveland.into()).eq(lit(false)))
                .count()
                .alias("Wavedashes"),
            col(clm::Waveland.into())
                .filter(col(clm::Waveland.into()).eq(lit(true)))
                .count()
                .alias("Wavelands"),
            col(clm::Angle.into()).mean().alias("AvgAngle"),
        ])
        .collect()
        .ok()
    }

    fn lc_summary(&self) -> Option<DataFrame> {
        use crate::columns::LCancelStats as clm;
        if let Some(df) = &self.l_cancel {
            let lf = df.clone().lazy();

            return lf.select(&[
                col(clm::LCancelled.into()).mean().alias("LCancelPercent"),
            ]).collect().ok();
        }

        None
    }

    fn def_summary(&self) -> Option<DataFrame> {
        use crate::columns::DefenseStats as clm;
        if let Some(df) = &self.defense {
            let lf = df.clone().lazy();
            return lf
                .select(&[
                    col(clm::DamageTaken.into()).sum(),
                    col(clm::DamageTaken.into()).count().alias("HitsTaken"),
                    col(clm::DIEfficacy.into()).mean(),
                    col(clm::LastHitBy.into())
                        .mode()
                        .implode()
                        .cast(DataType::List(Box::new(DataType::Utf8)))
                        .alias("MostHitBy"),
                    col(clm::StateBeforeHit.into())
                        .mode()
                        .implode()
                        .cast(DataType::List(Box::new(DataType::Utf8)))
                        .alias("StateMostPunished"),
                    col(clm::SDIInputs.into())
                        .list()
                        .len()
                        .sum()
                        .cast(DataType::Float32)
                        .div(lit(df.height() as u64))
                        .alias("SDIPerHit"),
                    col(clm::KillsWithDI.into())
                        .filter(
                            col(clm::KillsSomeDI.into())
                                .eq(lit(true))
                                .and(col(clm::KillsAllDI.into()).eq(lit(false))),
                        )
                        .mean()
                        .alias("LivableHitsLived"),
                    col(clm::KillsWithDI.into())
                        .filter(
                            col(clm::KillsSomeDI.into())
                                .eq(lit(true))
                                .and(col(clm::KillsAllDI.into()).eq(lit(false))),
                        )
                        .count()
                        .alias("LivableHits"),
                    col(clm::HitlagFrames.into()).sum().alias("FramesInHitlag"),
                    // col(clm::KillsWithDI.into())
                    //     .filter(
                    //         col(clm::KillsWithDI.into())
                    //             .eq(lit(false))
                    //             .and(col(clm::KillsSomeDI.into()).eq(lit(true)))
                    //     )
                    //     .count()
                    //     .alias("DILives")
                    //     ,
                    // col(clm::KillsSomeDI.into()).filter(col(clm::KillsSomeDI.into()).eq(lit(true))).count().alias("CouldDie")
                ])
                .collect()
                .ok();
        }

        None
    }
}

impl From<&[Arc<Stats>]> for Stats {
    fn from(values: &[Arc<Stats>]) -> Self {
        let mut input = DataFrame::default();
        let mut wavedash = DataFrame::default();
        let mut l_cancel = DataFrame::default();
        let mut tech = DataFrame::default();
        let mut item = DataFrame::default();
        let mut defense = DataFrame::default();

        for val in values {
            input = input.vstack(&val.input).unwrap();
            wavedash = wavedash.vstack(&val.wavedash).unwrap();
            if let Some(lc) = &val.l_cancel {
                l_cancel = l_cancel.vstack(lc).unwrap();
            }
            if let Some(th) = &val.tech {
                tech = tech.vstack(th).unwrap();
            }
            if let Some(it) = &val.item {
                item = item.vstack(it).unwrap();
            }
            if let Some(d) = &val.defense {
                defense = defense.vstack(d).unwrap();
            }
        }

        Stats {
            input,
            wavedash,
            l_cancel: match l_cancel.height() {
                0 => None,
                _ => Some(l_cancel),
            },
            tech: match tech.height() {
                0 => None,
                _ => Some(tech),
            },
            item: match item.height() {
                0 => None,
                _ => Some(item),
            },
            defense: match defense.height() {
                0 => None,
                _ => Some(defense),
            },
        }
    }
}

impl From<Vec<Arc<Stats>>> for Stats {
    fn from(value: Vec<Arc<Stats>>) -> Self {
        Stats::from(value.as_slice())
    }
}

#[derive(Debug, Copy, Clone, EnumString, IntoStaticStr, PartialEq, Eq)]
#[strum(ascii_case_insensitive)]
pub enum StatType {
    Input,
    Wavedash,
    #[strum(serialize = "L Cancel")]
    #[strum(serialize = "LCancel")]
    #[strum(serialize = "L_Cancel")]
    LCancel,
    Tech,
    Item,
    Defense,
}
