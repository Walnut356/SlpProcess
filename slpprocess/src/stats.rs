pub mod combos;
pub mod defense;
pub mod inputs;
pub mod items;
pub mod lcancel;
pub mod wavedash;

use polars::prelude::*;
use strum_macros::{IntoStaticStr, EnumString};

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

    pub fn get_summary(&self, stat_type: StatType) -> Option<DataFrame> {
        match stat_type {
            // These are already basically summaries anyway
            StatType::Input => Some(self.input.clone()),
            StatType::Item => self.item.clone(),

            StatType::Wavedash => todo!(),
            StatType::LCancel => todo!(),
            StatType::Defense => todo!(),
        }
    }

    fn wd_summary(&self) -> Option<DataFrame> {
        
        todo!()
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
                _ => Some(l_cancel)
            },
            item: match item.height() {
                0 => None,
                _ => Some(item)
            },
            defense: match defense.height() {
                0 => None,
                _ => Some(defense)
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
    #[strum(serialize="L Cancel")]
    #[strum(serialize="LCancel")]
    #[strum(serialize="L_Cancel")]
    LCancel,
    Item,
    Defense,
}