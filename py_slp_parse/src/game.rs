#![allow(non_camel_case_types)]

use std::{collections::HashMap, path::Path};

use crate::{
    frames::{PyFrames, PyItem},
    player::*,
};

use pyo3::prelude::*;
use slp_parse::prelude::*;

#[pyclass(name = "Game", frozen)]
pub struct PyGame {
    game: Game,
    #[pyo3(get)]
    players: Vec<PyPlayer>,
}

impl PyGame {
    pub fn new(game: Game) -> Self {
        let players = game
            .players
            .iter()
            .map(|x| PyPlayer::new(x.clone()))
            .collect();
        PyGame { game, players }
    }
}

#[pymethods]
impl PyGame {
    #[new]
    pub fn __init__(path: String) -> Self {
        let f_path = Path::new(&path);
        let game = Game::new(f_path).unwrap();
        PyGame::new(game)
    }

    pub fn get_port_frames(&self, port: usize) -> PyResult<PyFrames> {
        Ok(self.players.get(port).unwrap().frames.clone())
    }
    // --------------------------------------------- game start getters --------------------------------------------- //
    #[getter]
    pub fn get_random_seed(&self) -> PyResult<u32> {
        Ok(self.game.random_seed())
    }
    #[getter]
    pub fn get_is_teams(&self) -> PyResult<bool> {
        Ok(self.game.teams())
    }
    #[getter]
    pub fn get_stage(&self) -> PyResult<u16> {
        Ok(self.game.stage() as u16)
    }
    #[getter]
    pub fn get_timer(&self) -> PyResult<u64> {
        Ok(self.game.timer().as_secs())
    }
    #[getter]
    pub fn get_damage_ratio(&self) -> PyResult<f32> {
        Ok(self.game.damage_ratio())
    }
    #[getter]
    pub fn get_is_pal(&self) -> PyResult<Option<bool>> {
        Ok(self.game.pal())
    }
    #[getter]
    pub fn get_is_frozen_stadium(&self) -> PyResult<Option<bool>> {
        Ok(self.game.frozen_stadium())
    }
    #[getter]
    pub fn get_is_netplay(&self) -> PyResult<Option<bool>> {
        Ok(self.game.netplay())
    }
    #[getter]
    pub fn get_match_id(&self) -> PyResult<String> {
        Ok(self.game.match_id().to_string())
    }
    #[getter]
    pub fn get_match_type(&self) -> PyResult<u8> {
        Ok(self.game.match_type() as u8)
    }
    #[getter]
    pub fn get_game_number(&self) -> PyResult<Option<u32>> {
        Ok(self.game.game_number())
    }
    #[getter]
    pub fn get_tiebreak_number(&self) -> PyResult<Option<u32>> {
        Ok(self.game.tiebreak_number())
    }
    #[getter]
    pub fn get_item_frames(&self) -> PyResult<Option<PyItem>> {
        Ok(self
            .game
            .item_frames
            .as_ref()
            .map(|x| PyItem { frames: x.clone() }))
    }

    // ---------------------------------------------- game end getters ---------------------------------------------- //
    #[getter]
    pub fn get_end_method(&self) -> PyResult<Option<u8>> {
        match self.game.end() {
            Some(end) => Ok(Some(end.end_method.clone() as u8)),
            _ => Ok(None),
        }
    }
    #[getter]
    pub fn get_lras_initiator(&self) -> PyResult<Option<i8>> {
        match self.game.end() {
            Some(end) => Ok(end.lras_initiator.as_ref().map(|x| *x as i8)),
            _ => Ok(None),
        }
    }
    #[getter]
    pub fn get_placements(&self) -> PyResult<Option<HashMap<i8, i8>>> {
        match self.game.end() {
            Some(end) => Ok(end.placements.as_ref().map(|x| {
                // dear god
                x.iter()
                    .map(|(x, y)| (*x as i8, *y as i8)).collect::<HashMap<i8, i8>>()
            })),
            _ => Ok(None),
        }
    }
}
