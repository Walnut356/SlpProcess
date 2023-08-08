#![allow(non_camel_case_types)]

use std::path::Path;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use slpprocess::Game;

#[pyclass(name = "Frames")]
pub struct PyFrames {
    #[pyo3(get)]
    pre: PyDataFrame,
    #[pyo3(get)]
    post: PyDataFrame,
}

#[pyclass(name = "Game")]
pub struct PyGame {
    game: Game,
}

impl PyGame {
    pub fn new(game: Game) -> Self {
        PyGame { game }
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

    pub fn get_port_frames(&self, port: u8) -> PyResult<PyFrames> {
        let normalized = (port - 1).try_into().unwrap();
        for player in self.game.players.iter() {
            if player.read().unwrap().port == normalized {
                return Ok(PyFrames {
                    pre: PyDataFrame(player.read().unwrap().frames.pre.clone()),
                    post: PyDataFrame(player.read().unwrap().frames.post.clone()),
                });
            }
        }
        PyResult::Err(PyValueError::new_err(format!("No player with port {port}")))
    }

    // --------------------------------------------- game start getters --------------------------------------------- //
    #[getter]
    pub fn get_random_seed(&self) -> PyResult<u32> {
        Ok(self.game.start.random_seed)
    }
    #[getter]
    pub fn get_is_teams(&self) -> PyResult<bool> {
        Ok(self.game.start.is_teams)
    }
    #[getter]
    pub fn get_stage(&self) -> PyResult<u16> {
        Ok(self.game.start.stage.into())
    }
    #[getter]
    pub fn get_timer(&self) -> PyResult<u64> {
        Ok(self.game.start.timer.as_secs())
    }
    #[getter]
    pub fn get_damage_ratio(&self) -> PyResult<f32> {
        Ok(self.game.start.damage_ratio)
    }
    #[getter]
    pub fn get_is_pal(&self) -> PyResult<Option<bool>> {
        Ok(self.game.start.is_pal)
    }
    #[getter]
    pub fn get_is_frozen_stadium(&self) -> PyResult<Option<bool>> {
        Ok(self.game.start.is_frozen_stadium)
    }
    #[getter]
    pub fn get_is_netplay(&self) -> PyResult<Option<bool>> {
        Ok(self.game.start.is_netplay)
    }
    #[getter]
    pub fn get_match_id(&self) -> PyResult<Option<String>> {
        Ok(self.game.start.match_id.clone())
    }
    #[getter]
    pub fn get_match_type(&self) -> PyResult<Option<u8>> {
        Ok(self.game.start.match_type.map(|x| x as u8))
    }
    #[getter]
    pub fn get_game_number(&self) -> PyResult<Option<u32>> {
        Ok(self.game.start.game_number)
    }
    #[getter]
    pub fn get_tiebreak_number(&self) -> PyResult<Option<u32>> {
        Ok(self.game.start.tiebreak_number)
    }

    // ---------------------------------------------- game end getters ---------------------------------------------- //
    #[getter]
    pub fn get_end_method(&self) -> PyResult<Option<u8>> {
        match self.game.end.as_ref() {
            Some(end) => Ok(Some(end.end_method.clone() as u8)),
            _ => Ok(None),
        }
    }
    #[getter]
    pub fn get_lras_initiator(&self) -> PyResult<Option<i8>> {
        match self.game.end.as_ref() {
            Some(end) => Ok(end.lras_initiator),
            _ => Ok(None),
        }
    }
    #[getter]
    pub fn get_placements(&self) -> PyResult<Option<[i8; 4]>> {
        match self.game.end.as_ref() {
            Some(end) => Ok(end.placements),
            _ => Ok(None),
        }
    }
}