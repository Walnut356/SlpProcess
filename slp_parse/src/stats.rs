pub(crate) mod combos;
pub(crate) mod defense;
pub(crate) mod inputs;
pub(crate) mod items;
pub(crate) mod lcancel;
// pub(crate) mod recovery;
pub(crate) mod tech;
pub(crate) mod wavedash;

use std::sync::Arc;

pub use combos::Combos;
pub use defense::DefenseStats;
pub use inputs::InputStats;
pub use items::ItemStats;
pub use lcancel::LCancelStats;
// pub use recovery::RecoveryStats;
pub use tech::TechStats;
pub use wavedash::WavedashStats;

use strum_macros::{EnumString, IntoStaticStr};

use crate::game::Metadata;

pub trait Stat {}

#[derive(Debug, Copy, Clone, EnumString, IntoStaticStr, PartialEq, Eq, strum_macros::Display)]
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

// TODO unify the way stats are instantiated. Some translate from rows, some have the columns built
// manually and then get .into_box()'d

#[derive(Debug, Default, Clone)]
pub struct Stats {
    /// Minimum Replay Version: Any
    pub input: InputStats,
    /// Minimum Replay Version: Any
    pub wavedash: WavedashStats,
    /// Minimum Replay Version: 2.0.0
    pub l_cancel: Option<LCancelStats>,
    /// Minimum Replay Version: 2.0.0
    pub tech: Option<TechStats>,
    /// Minimum Replay Version: 3.0.0
    pub item: Option<ItemStats>,
    /// Minimum Replay Version 3.5.0
    pub defense: Option<DefenseStats>,
}

// Eventually maybe i'll use this and add convenience methods for bulk parsing
// #[derive(Debug, Default, Clone)]
// pub struct StatsList {
//     pub input: Vec<InputStats>,
//     pub wavedash: Vec<WavedashStats>,
//     pub l_cancel: Vec<Option<LCancelStats>>,
//     pub tech: Vec<Option<TechStats>>,
//     pub item: Vec<Option<ItemStats>>,
//     pub defense: Vec<Option<DefenseStats>>,
// }

// impl<'a> FromIterator<&'a Stats> for StatsList {
//     fn from_iter<T: IntoIterator<Item = &'a Stats>>(iter: T) -> Self {
//         Self {
//             input: iter.into_iter().map(|x| x.input).collect::<Vec<_>>(),
//             wavedash: iter.into_iter().map(|x| x.wavedash).collect::<Vec<_>>(),
//             l_cancel: iter.into_iter().map(|x| x.l_cancel).collect::<Vec<_>>(),
//             tech: iter.into_iter().map(|x| x.tech).collect::<Vec<_>>(),
//             item: iter.into_iter().map(|x| x.item).collect::<Vec<_>>(),
//             defense: iter.into_iter().map(|x| x.defense).collect::<Vec<_>>(),
//         }
//     }
// }
