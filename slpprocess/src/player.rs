use std::sync::Arc;

use ssbm_utils::enums::{character::Costume, Character, Port};

use crate::{
    events::game_start::ControllerFix,
    frames::Frames,
    stats::{combos::Combos, Stats},
};

///
#[derive(Debug, Default)]
pub struct Player {
    /// In-game character, can be translated to in-game or character select screen raw value via
    /// `.as_internal()` and `try_as_css()`
    pub character: Character,
    /// Character's interal costume value
    pub costume: Costume,
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

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerStub {
    /// In-game character, can be translated to in-game or character select screen raw value via
    /// `.as_internal()` and `try_as_css()`
    pub character: Character,
    /// Character's interal costume value
    pub costume: Costume,
    /// Player's port number P1-P4. Can be cast into 0-indexed u8 port number via `as u8`
    pub port: Port,
    /// Player's connect code (if netplay) in the form "CODE#123"
    pub connect_code: Option<String>,
    /// Player's display name (if netplay). Has a max length of 15 characters (or 30 bytes)
    pub display_name: Option<String>,
}

impl From<Player> for PlayerStub {
    fn from(value: Player) -> Self {
        Self {
            character: value.character,
            costume: value.costume,
            port: value.port,
            connect_code: value.connect_code,
            display_name: value.display_name,
        }
    }
}
