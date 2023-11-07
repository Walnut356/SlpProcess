use std::sync::Arc;

use polars::prelude::DataFrame;
use ssbm_utils::enums::{Character, Port};

use crate::{
    events::{
        game_start::ControllerFix,
        post_frame::{PostFrames, PostRow},
        pre_frame::{PreFrames, PreRow},
    },
    stats::combos::Combos,
};

///
#[derive(Debug, Default)]
pub struct Player {
    /// In-game character, can be translated to in-game or character select screen raw value via
    /// `.as_internal()` and `try_as_css()`
    pub character: Character,
    /// Character's interal costume value
    pub costume: u8,
    /// Player's port number P1-P4. Can be cast into 0-indexed u8 port number via `as u8`
    pub port: Port,
    /// Player's connect code (if netplay) in the form "CODE#123"
    pub connect_code: Option<String>,
    /// Player's display name (if netplay). Has a max length of 15 characters (or 30 bytes)
    pub display_name: Option<String>,
    /// True if this player won the game, false if not. Can be None if the internal checks fail to
    /// determine a winner
    pub is_winner: Option<bool>,
    /// UCF/Arduino information for this port.
    pub ucf: Option<UCFToggles>,
    /// Container for stat containers
    pub stats: Arc<Stats>,
    /// Container for all combos detected for this player during the match
    pub combos: Arc<Combos>,
    /// Container for Pre and Post frame containers
    pub frames: Frames,
    /// None if Player.character is not Ice Climbers, otherwise contains Nana's `Frames` object.
    pub nana_frames: Option<Frames>,
}

/// Records information on which Dashback and Shielddrop toggles are activated. Possible values for
/// each are `UCF`, `Dween` and `Off`
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct UCFToggles {
    pub dashback: ControllerFix,
    pub shield_drop: ControllerFix,
}

/// Container for Pre-frame and Post-frame containers.
///
/// Note that frames are stored in columnar format, meaning data access is as follows:
/// `player.frames.post.acion_state[index]`
///
/// `.get_frame(index)` functions exist for `Frames`, `PreFrames`, and `PostFrames` objects, but
/// these will generally be much slower than iterating through only the columns you need.
#[derive(Debug, Default, Clone)]
pub struct Frames {
    pub pre: Arc<PreFrames>,
    pub post: Arc<PostFrames>,
}

impl Frames {
    pub fn len(&self) -> usize {
        self.pre.frame_index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() > 0
    }

    /// Gets both the full pre-frame and post-frame for a given frame index (0-indexed). This is very
    /// slow compared to iterating through only the columns you need.
    pub fn get_frame(&self, index: usize) -> (PreRow, PostRow) {
        (self.pre.get_frame(index), self.post.get_frame(index))
    }
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
