use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, ensure, Result};
use itertools::Itertools;

use polars::prelude::*;
use ssbm_utils::enums::StageID;
use ssbm_utils::enums::{stage::Stage, Port};
use time::OffsetDateTime;

use crate::events::game_start::MatchType;
use crate::{
    events::{
        game_end::{EndMethod, GameEnd},
        game_start::{GameStart, Version},
        item_frames::ItemFrames,
    },
    player::{Player, PlayerStub},
    stats::{
        combos::find_combos, defense::find_defense, inputs::find_inputs, items::find_items,
        lcancel::find_lcancels, tech::find_techs, wavedash::find_wavedashes, Stats,
    },
};

pub trait GameMetadata {
    fn metadata(&self) -> &GameStart;

    fn date(&self) -> OffsetDateTime;

    fn duration(&self) -> Duration;

    fn version(&self) -> Version;

    fn path(&self) -> Arc<PathBuf>;

    #[inline]
    fn random_seed(&self) -> u32 {
        self.metadata().random_seed
    }

    #[inline]
    fn is_teams(&self) -> bool {
        self.metadata().teams
    }

    #[inline]
    fn stage(&self) -> StageID {
        self.metadata().stage
    }

    #[inline]
    fn timer(&self) -> Duration {
        self.metadata().timer
    }

    #[inline]
    fn damage_ratio(&self) -> f32 {
        self.metadata().damage_ratio
    }

    #[inline]
    fn pal(&self) -> Option<bool> {
        self.metadata().pal
    }

    #[inline]
    fn frozen_stadium(&self) -> Option<bool> {
        self.metadata().frozen_stadium
    }

    #[inline]
    fn netplay(&self) -> Option<bool> {
        self.metadata().netplay
    }

    #[inline]
    fn match_id(&self) -> Arc<String> {
        self.metadata().match_id.clone()
    }

    #[inline]
    fn match_type(&self) -> MatchType {
        self.metadata().match_type
    }

    #[inline]
    fn game_number(&self) -> Option<u32> {
        self.metadata().game_number
    }

    #[inline]
    fn tiebreak_number(&self) -> Option<u32> {
        self.metadata().tiebreak_number
    }
}

#[derive(Clone)]
pub struct Game {
    /// Slippi-spec GameStart event. Contains various data about the match itself
    /// including stage, match id, etc.
    pub metadata: GameStart,
    /// Slippi-spec GameEnd event. Contains info about how the game ended.
    ///
    /// Can be `None`, even on modern replays due to an unresolved bug where the event sometimes
    /// doesn't populate.
    pub end: Option<GameEnd>, // There's an unresolved bug where sometiems game end events don't appear
    /// Duration of the game, accurate to the **ingame timer**. For the -123 indexed total frame
    /// count, see `.total_frames`
    pub duration: Duration,
    /// A flat number equal to the total number of frames in the replay.
    pub total_frames: usize,
    /// Replay SemVer number in the form `major`, `minor`, `revision`
    pub version: Version,
    /// Contains exactly 2 Players in threadsafe containers (`.load()` to access). Players are in
    /// port order, but may be any combination of ports. Port numbers are stored in the Player
    /// objects
    pub players: [Arc<Player>; 2],
    /// Contains Item Frames if the replay is new enough. Item frames themselves may be empty if no
    /// items spawned during the match (highly unlikely), but the container will populate so long as
    /// the replay is new enough
    pub item_frames: Option<Arc<ItemFrames>>,
    /// The full path of the parsed replay.
    ///
    /// Used internally for generating Dolphin Playback Queues.
    pub path: Arc<PathBuf>,
    /// Datetime the match was played on. Defaults to the UNIX epoch time (Midnight, 1 January, 1970 (UTC))
    ///
    /// added v0.1.0
    pub date: OffsetDateTime,
    /// Difference between the total number of frames present in the game, and the final framecount
    /// of the game. Useful for checking how laggy a match was.
    pub frames_rollbacked: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            metadata: Default::default(),
            end: Default::default(),
            duration: Default::default(),
            total_frames: Default::default(),
            version: Default::default(),
            players: Default::default(),
            item_frames: Default::default(),
            path: Default::default(),
            date: OffsetDateTime::UNIX_EPOCH,
            frames_rollbacked: Default::default(),
        }
    }
}

impl Game {
    /// Creates a new game object from the given Path.
    ///
    /// Can panic if replay is severely malformed (Payload size doesn't match Payload Sizes listing,
    /// metadata event missing, etc.)
    pub fn new(path: &Path) -> Result<Self> {
        ensure!(
            path.is_file() && path.extension().unwrap() == "slp",
            "Expected file with extension .slp, got path: {path:?}"
        );
        let file_data = Self::get_file_contents(path)?;
        let mut game = Game::parse(file_data, path)?;
        // let now = Instant::now();
        game.get_stats();
        // let dur = now.elapsed();
        // println!("{:?}", dur);

        Ok(game)
    }

    pub fn player_by_port(&self, port: Port) -> Result<Arc<Player>> {
        for p_lock in self.players.iter() {
            // TODO i don't like making this a string, but RwLock errors require Sync/Send so
            // they can't be directly converted to anyhow errors without some fiddling. I don't
            // feel super confident about that, but it's also not very important atm so i'll
            // look into it later
            let player = p_lock;

            if player.port == port {
                return Ok(player.clone());
            }
        }

        Err(anyhow!("Unable to find player with port {:?}", port))
    }

    pub fn player_by_code(&self, connect_code: &str) -> Result<Arc<Player>> {
        for p_lock in self.players.iter() {
            let player = p_lock;

            if player
                .connect_code
                .as_ref()
                .is_some_and(|x| x.to_ascii_uppercase() == connect_code.to_ascii_uppercase())
            {
                return Ok(player.clone());
            }
        }

        Err(anyhow!(
            "Unable to find player with connect code '{:?}'",
            connect_code
        ))
    }

    /// Replaces the `frames` object of each player with `Default::default()`. Used to save memory
    /// when frames are not going to be analyzed further. Since frames are in an Arc, this only guarantees
    /// freeing memory if there are no other outstanding references.
    pub fn drop_frames(&mut self) {
        let mut result = Vec::new();
        for player in &self.players {
            result.push(Arc::new(Player {
                character: player.character,
                costume: player.costume,
                port: player.port,
                connect_code: player.connect_code.clone(),
                display_name: player.display_name.clone(),
                is_winner: self.get_winner().map(|x| x == player.port),
                ucf: player.ucf,
                stats: player.stats.clone(),
                combos: player.combos.clone(),
                frames: Default::default(),
                nana_frames: player.nana_frames.clone(),
            }));
        }

        self.players = result.try_into().unwrap();
    }

    pub fn summarize(&self) -> DataFrame {
        // I fucking hate time libraries so much. All of them have ergonomics like this.
        let date = self.date;
        // .to_offset(time::UtcOffset::current_local_offset().unwrap());
        let offset = time::UtcOffset::current_local_offset().unwrap();

        let v_s = vec![
            // changing the offset doesn't actually change the underlying value so we have to do it manually =)
            Series::new(
                "Datetime",
                &[AnyValue::Datetime(
                    // manually coerce offset into seconds and seconds into milliseconds
                    (date.unix_timestamp() + (offset.whole_hours() as i64 * 3600)) * 1000,
                    TimeUnit::Milliseconds,
                    &None,
                )],
            ),
            Series::new(
                "Duration",
                &[AnyValue::Duration(
                    self.duration.as_millis() as i64,
                    TimeUnit::Milliseconds,
                )],
            ),
            Series::new("MatchType", &[Into::<&str>::into(self.metadata.match_type)]),
            Series::new("Game", &[self.metadata.game_number]),
            Series::new("Tiebreak", &[self.metadata.tiebreak_number]),
            Series::new("Stage", &[Into::<&str>::into(self.metadata.stage)]),
        ];

        DataFrame::new(v_s).unwrap()
        // let df = DataFrame::(
        //     "File" => &[self.path.file_stem().unwrap().to_str()],
        //     "Datetime" => &[date.unix_timestamp()],
        //     "Duration" => &[self.duration.as_millis() as u64],
        //     // "MatchID" => &[self.metadata.match_id.clone()],
        //     "MatchType" => &[self.metadata.match_type.map(Into::<&str>::into)],
        //     "Game" => &[self.metadata.game_number],
        //     "Tiebreak" => &[self.metadata.tiebreak_number],
        //     "Stage" => &[Into::<&str>::into(self.metadata.stage)],

        // ).unwrap();
    }

    fn get_stats(&mut self) {
        let version = self.version;
        let mut result: Vec<Arc<Player>> = Vec::new();

        let stage = Stage::from_id(self.metadata.stage);

        for players in self.players.iter().permutations(2) {
            // inner scope for read-only operations
            let player = players[0];
            let opponent = players[1];
            let items = &self.item_frames;

            // inputs are available in every replay version
            let input = find_inputs(&player.frames, self.total_frames);

            // l cancel status was with 2.0.0 on 3/19/2019
            let l_cancel = version
                .at_least(2, 0, 0)
                .then(|| find_lcancels(&player.frames, &stage));

            let tech = version
                .at_least(2, 0, 0)
                .then(|| find_techs(&player.frames, &opponent.frames, &stage));

            // requires fields up to item.owner which was released just after rollback on 7/8/2020
            let item = version
                .at_least(3, 6, 0)
                .then(|| find_items(player.port, items.as_ref().unwrap()));

            // requires knockback speed values which requires v3.5.0, released just before rollback
            // on 6/20/2020
            let defense = version.at_least(3, 5, 0).then(|| {
                find_defense(
                    &player.frames,
                    &opponent.frames,
                    self.metadata.stage as u16,
                    player.character,
                    opponent.character,
                )
            });

            // requires inputs and states thus has no version requirement
            let wavedash = find_wavedashes(&player.frames);

            let stats = Arc::new(Stats {
                input,
                l_cancel,
                item,
                defense,
                wavedash,
                tech,
            });

            let combos = Arc::new(find_combos(
                &player.frames,
                &opponent.frames,
                self.metadata.stage,
                player.character,
                self.path.clone(),
            ));

            /* This should be a pretty cheap clone all things considered. The frames are 2 Arc
            clones, and the connect code/display name are a max of 40 bytes (max 10 for code, max
            30 for display name)

            I'm not super happy with how this turned out, but the ergonomics for the end-user are
            nicer than Arc<RwLock<>> or RwLock<Arc<>>.

            The alternative (i think?) is to use something like OnceLock<Arc<Stats>>, which i may
            still change to later. It's a negligable performance impact to save typing .get()
            and i'm lazy =)
             */

            result.push(Arc::new(Player {
                character: player.character,
                costume: player.costume,
                port: player.port,
                connect_code: player.connect_code.clone(),
                display_name: player.display_name.clone(),
                is_winner: self.get_winner().map(|x| x == player.port),
                ucf: player.ucf,
                stats,
                combos,
                frames: player.frames.clone(),
                nana_frames: player.nana_frames.clone(),
            }));
        }

        assert_eq!(result.len(), 2);
        self.players = result.try_into().unwrap();
    }

    // pub fn get_combos(&mut self) {
    //     find_combos(plyr_frames, opnt_frames, stage_id, player_char)
    // }

    /// Returns the winner of the match if one can be decided conclusively
    pub fn get_winner(&self) -> Option<Port> {
        // I'm not sure a replay can even have 0 frames, but this saves us from possible panics
        // down the line
        if self.total_frames == 0 {
            return None;
        }

        let p1 = &self.players[0];
        let p2 = &self.players[1];

        // Anyone who LRAS's loses by default (matches slippi behavior)
        if self
            .end
            .as_ref()
            .is_some_and(|x| x.end_method == EndMethod::NoContest)
        {
            let lras = Port::from_repr(
                self.end
                    .as_ref()
                    .unwrap()
                    .lras_initiator
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .unwrap();

            if lras == p1.port {
                return Some(p2.port);
            } else if lras == p2.port {
                return Some(p1.port);
            } else {
                return None;
            }
        }

        let p1_stocks = *p1.frames.post.stocks.last().unwrap();
        let p2_stocks = *p2.frames.post.stocks.last().unwrap();

        /* I'm not sure whether or not percents are reset instantly upon dying, so this is a safety
            check. If both players die at the same time (and that ends the game), we assume it's a
            tie, thus we can't determine a winner.
        */
        if p1_stocks == 0 && p2_stocks == 0 {
            return None;
        }

        /* Otherwise, check to see who has more stocks, and then who has more percent on the last
            frame. This should handle regular game-end (loser will have 0 stocks on the last frame)
            timeouts (stock and percent check),
        */
        match p1_stocks.cmp(&p2_stocks) {
            std::cmp::Ordering::Less => Some(p2.port),
            std::cmp::Ordering::Greater => Some(p1.port),
            std::cmp::Ordering::Equal => {
                // The percent as seen on the HUD
                let p1_percent = p1.frames.post.percent.last().unwrap().floor();
                let p2_percent = p2.frames.post.percent.last().unwrap().floor();

                match p1_percent.partial_cmp(&p2_percent).unwrap() {
                    std::cmp::Ordering::Less => Some(p2.port),
                    std::cmp::Ordering::Greater => Some(p1.port),
                    std::cmp::Ordering::Equal => None,
                }
            }
        }
    }
}

impl GameMetadata for Game {
    #[inline]
    fn metadata(&self) -> &GameStart {
        &self.metadata
    }

    #[inline]
    fn date(&self) -> OffsetDateTime {
        self.date
    }

    #[inline]
    fn duration(&self) -> Duration {
        self.duration
    }

    #[inline]
    fn version(&self) -> Version {
        self.version
    }

    #[inline]
    fn path(&self) -> Arc<PathBuf> {
        self.path.clone()
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.date.unix_timestamp_nanos() == other.date.unix_timestamp_nanos()
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Game {}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date
            .unix_timestamp_nanos()
            .cmp(&other.date.unix_timestamp_nanos())
    }
}

#[derive(Debug, Clone)]
pub struct GameStub {
    pub metadata: GameStart,
    pub duration: Duration,
    pub version: Version,
    pub players: [PlayerStub; 2],
    pub path: Arc<PathBuf>,
    pub date: OffsetDateTime,
}

impl GameMetadata for GameStub {
    #[inline]
    fn metadata(&self) -> &GameStart {
        &self.metadata
    }
    #[inline]
    fn date(&self) -> OffsetDateTime {
        self.date
    }
    #[inline]
    fn duration(&self) -> Duration {
        self.duration
    }

    #[inline]
    fn version(&self) -> Version {
        self.version
    }

    #[inline]
    fn path(&self) -> Arc<PathBuf> {
        self.path.clone()
    }
}

impl From<GameStub> for Game {
    fn from(value: GameStub) -> Self {
        // should be fine to unwrap. If you can get a stub, the game parsing shouldn't panic
        Self::new(&value.path).unwrap()
    }
}

impl From<&GameStub> for Game {
    fn from(value: &GameStub) -> Self {
        Self::new(&value.path).unwrap()
    }
}

impl PartialEq for GameStub {
    fn eq(&self, other: &Self) -> bool {
        self.date.unix_timestamp_nanos() == other.date.unix_timestamp_nanos()
    }
}

impl PartialOrd for GameStub {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for GameStub {}

impl Ord for GameStub {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date
            .unix_timestamp_nanos()
            .cmp(&other.date.unix_timestamp_nanos())
    }
}
