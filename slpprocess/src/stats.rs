pub mod combos;
pub mod defense;
pub mod inputs;
pub mod items;
pub mod lcancel;
pub mod wavedash;

use std::cell::OnceCell;

use polars::prelude::*;
use strum_macros::{EnumString, IntoStaticStr};

macro_rules! into_str {
    ($x:expr) => {
        Into::<&'static str>::into($x)
    };
}

#[derive(Debug, Default, Clone)]
pub struct Stats {
    /// Minimum Replay Version: Any
    pub input: DataFrame,
    /// Minimum Replay Version: Any
    pub wavedash: DataFrame,
    /// Minimum Replay Version: 2.0
    pub l_cancel: Option<DataFrame>,
    /// Minimum Replay Version: 3.0
    pub item: Option<DataFrame>,
    /// Minimum Replay Version 3.5
    pub defense: Option<DataFrame>,
}

impl Stats {
    /// Returns an owned stat object
    pub fn get(&self, stat_type: StatType) -> Option<DataFrame> {
        match stat_type {
            StatType::Input => Some(self.input.clone()),
            StatType::Wavedash => Some(self.wavedash.clone()),
            StatType::LCancel => self.l_cancel.clone(),
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
            StatType::Defense => self.def_summary(),
        }
    }

    // And below is where we learn why I don't really like polars' Rust api. Extracting individual
    // values is incredibly verbose even if you know exactly what you want. The price we pay for
    // AnyValue I suppose

    fn wd_summary(&self) -> Option<DataFrame> {
        use crate::columns::WavedashStats as col;
        let avg_angle = self
            .wavedash
            .column(Into::<&'static str>::into(col::Angle))
            .unwrap()
            .f32()
            .unwrap()
            .mean()
            .unwrap(); // select([Into::<&str>::into(col::Angle)]).unwrap().mean();

        df!(
            "WavedashCount" => &[self.wavedash.height() as u32],
            "AvgAngle" => &[avg_angle as f32]
        )
        .ok()
    }

    fn lc_summary(&self) -> Option<DataFrame> {
        use crate::columns::LCancelStats as col;
        if let Some(df) = &self.l_cancel {
            let temp = df
                .column(into_str!(col::LCancelled))
                .unwrap()
                .bool()
                .unwrap()
                .mean();

            return df!(
                "LCancelEvents" => &[df.height() as u32],
                "LCancelPercent" => &[temp],
            )
            .ok();
        }

        None
    }

    fn def_summary(&self) -> Option<DataFrame> {
        use crate::columns::DefenseStats as col;
        if let Some(df) = &self.defense {
            let total_dmg = df
                .column(into_str!(col::DamageTaken))
                .unwrap()
                .f32()
                .unwrap()
                .sum();
            let common_hit = {
                // if these aren't 2 different statements the borrow checker gets mad =')
                let temp_1 = mode::mode(df.column(into_str!(col::LastHitBy)).unwrap()).unwrap();
                let temp_2 = temp_1.get(0);
                match temp_2 {
                    Ok(AnyValue::Utf8(x)) => x.to_owned(),
                    Err(_) => "null".to_string(),
                    _ => panic!("Non-string value in column LastHitBy"),
                }
            };
            let avg_sdi = if df.height() > 0 {
                let temp = df.column(into_str!(col::SDIInputs)).unwrap();
                let mut total = 0;
                for val in temp.iter() {
                    match val {
                        AnyValue::List(x) => total += x.len(),
                        _ => panic!("Non-list value in column SDIInputs"),
                    }
                }
                total as f32 / df.height() as f32
            } else {
                0.0
            };

            return Some(
                df![
                    "TotalDamage" => &[total_dmg],
                    "MostHitBy" => &[common_hit],
                    "SDIPerHit" => &[avg_sdi as f32],
                ]
                .unwrap(),
            );
        }

        None
    }
}

impl From<&[Arc<Stats>]> for Stats {
    fn from(values: &[Arc<Stats>]) -> Self {
        let mut input = DataFrame::default();
        let mut wavedash = DataFrame::default();
        let mut l_cancel = DataFrame::default();
        let mut item = DataFrame::default();
        let mut defense = DataFrame::default();

        for val in values {
            input = input.vstack(&val.input).unwrap();
            wavedash = wavedash.vstack(&val.wavedash).unwrap();
            if let Some(lc) = &val.l_cancel {
                l_cancel = l_cancel.vstack(lc).unwrap();
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
    Item,
    Defense,
}
