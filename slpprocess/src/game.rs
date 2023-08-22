use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use anyhow::{ensure, Result, anyhow};
use polars::prelude::DataFrame;

use crate::Port;
use crate::{
    events::{
        game_end::GameEnd,
        game_start::{GameStart, Version},
    },
    player::Player,
};

pub struct Game {
    pub start: GameStart,
    pub end: Option<GameEnd>, // There's an unresolved bug where sometiems game end events don't appear
    /// Duration of the game, accurate to the **ingame timer**. For the -123 indexed total frame
    /// count, see `.total_frames`
    pub duration: Duration,
    pub total_frames: u64,
    pub version: Version,
    pub players: [Arc<RwLock<Player>>; 2],
    pub item_frames: DataFrame,
}

impl Game {
    /// Creates a new game object from the given Path.
    ///
    /// Can panic if replay is severely malformed (Payload size doesn't match Payload Sizes listing,
    /// metadata event missing, etc.)
    pub fn new(path: &Path) -> Result<Self> {
        ensure!(path.is_file() && path.extension().unwrap() == "slp");
        let file_data = Self::get_file_contents(path)?;
        Game::parse(file_data)
    }

    pub fn get_port(&self, port: Port) -> Result<&RwLock<Player>> {
        for player in self.players.as_ref().iter() {
            if player.read().unwrap().port == port {
                return Ok(player);
            }
        }

        Err(anyhow!("Unable to find player with port {:?}", port))
    }

    pub fn get_port_mut(&mut self, port: Port) -> Result<&RwLock<Player>> {
        for player in self.players.iter() {
            if player.write().unwrap().port == port {
                return Ok(player);
            }
        }

        Err(anyhow!("Unable to find player with port {:?}", port))
    }
}
