use std::path::Path;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant};

use anyhow::{anyhow, ensure, Result};
use polars::prelude::DataFrame;

use crate::stats::helpers::get_stats;
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
        let mut game = Game::parse(file_data)?;
        // let now = Instant::now();
        get_stats(&mut game);
        // let dur = now.elapsed();
        // println!("{:?}", dur);

        Ok(game)
    }

    pub fn player_by_port(&self, port: Port) -> Result<RwLockReadGuard<'_, Player>> {
        for p_lock in self.players.as_ref().iter() {
            // TODO i don't like making this a string, but RwLock errors require Sync/Send so
            // they can't be directly converted to anyhow errors without some fiddling. I don't
            // feel super confident about that, but it's also not very important atm so i'll
            // look into it later
            let player = p_lock.read().map_err(|x| anyhow!("{:?}", x.to_string()))?;

            if player.port == port {
                return Ok(player);
            }
        }

        Err(anyhow!("Unable to find player with port {:?}", port))
    }

    pub fn player_by_code(&self, connect_code: &str) -> Result<RwLockReadGuard<'_, Player>> {
        for p_lock in self.players.iter() {
            let player = p_lock.read().map_err(|x| anyhow!("{:?}", x.to_string()))?;

            if player.connect_code.as_ref().is_some_and(|x| x.as_str() == connect_code) {
                return Ok(player);
            }
        }

        Err(anyhow!(
            "Unable to find player with connect code '{:?}'",
            connect_code
        ))
    }

    pub(crate) fn player_by_port_mut(
        &mut self,
        port: Port,
    ) -> Result<RwLockWriteGuard<'_, Player>> {
        for p_lock in self.players.iter() {
            let player = p_lock.write().map_err(|x| anyhow!("{:?}", x.to_string()))?;

            if player.port == port {
                return Ok(player);
            }
        }

        Err(anyhow!("Unable to find player with port {:?}", port))
    }
}
