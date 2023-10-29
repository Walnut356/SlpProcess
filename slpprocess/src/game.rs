use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, ensure, Result};
use arc_swap::{ArcSwap, Guard};
use itertools::Itertools;

use ssbm_utils::enums::{stage::Stage, Port};

use crate::{
    events::{
        game_end::GameEnd,
        game_start::{GameStart, Version},
        item_frames::ItemFrames,
    },
    player::{Player, Stats},
    stats::{
        combos::find_combos, defense::find_defense, inputs::find_inputs, items::find_items,
        lcancel::find_lcancels,
    },
};

pub struct Game {
    pub metadata: GameStart,
    pub end: Option<GameEnd>, // There's an unresolved bug where sometiems game end events don't appear
    /// Duration of the game, accurate to the **ingame timer**. For the -123 indexed total frame
    /// count, see `.total_frames`
    pub duration: Duration,
    pub total_frames: u64,
    pub version: Version,
    pub players: [ArcSwap<Player>; 2],
    pub item_frames: Option<Arc<ItemFrames>>,
    pub path: PathBuf,
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
        game.path = path.into();
        // let now = Instant::now();
        game.get_stats();
        // let dur = now.elapsed();
        // println!("{:?}", dur);

        Ok(game)
    }

    pub fn player_by_port(&self, port: Port) -> Result<Guard<Arc<Player>>> {
        for p_lock in self.players.as_ref().iter() {
            // TODO i don't like making this a string, but RwLock errors require Sync/Send so
            // they can't be directly converted to anyhow errors without some fiddling. I don't
            // feel super confident about that, but it's also not very important atm so i'll
            // look into it later
            let player = p_lock.load();

            if player.port == port {
                return Ok(player);
            }
        }

        Err(anyhow!("Unable to find player with port {:?}", port))
    }

    pub fn player_by_code(&self, connect_code: &str) -> Result<Guard<Arc<Player>>> {
        for p_lock in self.players.iter() {
            let player = p_lock.load();

            if player
                .connect_code
                .as_ref()
                .is_some_and(|x| x.as_str() == connect_code)
            {
                return Ok(player);
            }
        }

        Err(anyhow!(
            "Unable to find player with connect code '{:?}'",
            connect_code
        ))
    }

    pub fn get_stats(&mut self) {
        let version = self.version;

        for players in self.players.iter().permutations(2) {
            // inner scope for read-only operations
            let player = players[0].load();
            let opponent = players[1].load();
            let items = &self.item_frames;

            // inputs are available in every replay version
            let inputs = find_inputs(&player.frames, self.total_frames);

            // l cancel status was with 2.0.0 on 3/19/2019
            let l_cancel = version
                .at_least(2, 0, 0)
                .then(|| find_lcancels(&player.frames, &Stage::from_id(self.metadata.stage)));

            // requires fields up to item.owner which was released just after rollback on 7/8/2020
            let items = version
                .at_least(3, 6, 0)
                .then(|| find_items(&player.frames, player.port, items.as_ref().unwrap()));

            // requires knockback speed values which requires v3.5.0, released just before rollback
            // on 6/20/2020
            let defense = version.at_least(3, 5, 0).then(|| {
                find_defense(
                    &player.frames,
                    &opponent.frames,
                    self.metadata.stage as u16,
                    player.character,
                )
            });

            let stats = Arc::new(Stats {
                inputs,
                l_cancel,
                items,
                defense,
            });

            let combos = Arc::new(find_combos(
                &player.frames,
                &opponent.frames,
                self.metadata.stage,
                player.character,
            ));

            /* This should be a pretty cheap clone all things considered. The frames are 2 Arc
            clones, and the connect code/display name are a max of 40 bytes (max 10 for code, max
            30 for display name)

            I'm not super happy with how this turned out, but the ergonomics for the end-user are
            nicer than Arc<RwLock<>> or RwLock<Arc<>>and ArcSwap is heavily optimized for
            seldom-write, often-read which is exactly my usecase.

            The alternative (i think?) is to use something like OnceLock<Arc<Stats>>, which i may
            still change to later. It's a negligable performance impact to save typing .get()
            and i'm lazy =)
             */

            self.players[0].store(Arc::new(Player {
                character: player.character,
                costume: player.costume,
                port: player.port,
                connect_code: player.connect_code.clone(),
                display_name: player.display_name.clone(),
                is_winner: player.is_winner,
                ucf: player.ucf,
                stats,
                combos,
                frames: player.frames.clone(),
                nana_frames: player.nana_frames.clone(),
            }))
        }
    }

    // pub fn get_combos(&mut self) {
    //     find_combos(plyr_frames, opnt_frames, stage_id, player_char)
    // }
}
