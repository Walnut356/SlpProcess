#![allow(non_camel_case_types)]

use std::path::Path;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::*;
use pyo3_polars::PyDataFrame;
use serde::Serialize;
use slpprocess::events::game_start::GameStart;
use slpprocess::Game;

#[pyclass(name = "Game")]
pub struct PyGame {
    game: Game,
    #[pyo3(get)]
    start: PyGameStart,
    // end: PyGameEnd,
    // players: PyTuple,
}

impl PyGame {
    pub fn new(game: Game) -> Self {
        let start = PyGameStart::new(&game.start);
        PyGame { game, start }
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
            if player.port == normalized {
                return Ok(PyFrames {
                    pre: PyDataFrame(player.frames.pre.clone()),
                    post: PyDataFrame(player.frames.post.clone()),
                });
            }
        }
        PyResult::Err(PyValueError::new_err(format!("No player with port {port}")))
    }
}

#[pyclass(name = "Frames")]
pub struct PyFrames {
    #[pyo3(get)]
    pre: PyDataFrame,
    #[pyo3(get)]
    post: PyDataFrame,
}

#[derive(Clone, Serialize)]
#[pyclass(name = "GameStart")]
pub struct PyGameStart {
    #[pyo3(get)]
    random_seed: u32,
    #[pyo3(get)]
    is_teams: bool,
    #[pyo3(get)]
    stage: u16,
    #[pyo3(get)]
    timer: u64,
    #[pyo3(get)]
    damage_ratio: f32,
    #[pyo3(get)]
    is_pal: Option<bool>,
    #[pyo3(get)]
    is_frozen_stadium: Option<bool>,
    #[pyo3(get)]
    is_netplay: Option<bool>,
    #[pyo3(get)]
    match_id: Option<String>,
    #[pyo3(get)]
    match_type: Option<u8>,
    #[pyo3(get)]
    game_number: Option<u32>,
    #[pyo3(get)]
    tiebreak_number: Option<u32>,
}

#[pymethods]
impl PyGameStart {
    fn __repr__(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl PyGameStart {
    fn new(game_start: &GameStart) -> Self {
        PyGameStart {
            random_seed: game_start.random_seed,
            is_teams: game_start.is_teams,
            stage: game_start.stage.into(),
            timer: game_start.timer.as_secs(),
            damage_ratio: game_start.damage_ratio,
            is_pal: game_start.is_pal,
            is_frozen_stadium: game_start.is_frozen_stadium,
            is_netplay: game_start.is_netplay,
            match_id: game_start.match_id.as_ref().map(|x| x.to_string()),
            match_type: game_start.match_type.map(|x| x.into()),
            game_number: game_start.game_number,
            tiebreak_number: game_start.tiebreak_number,
        }
    }
}

#[pyfunction]
pub fn parse(path: String) -> Vec<PyGame> {
    let vals: Vec<Game> = slpprocess::parse(&path);

    vals.into_iter().map(PyGame::new).collect()
}

#[pymodule]
fn slp_process(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGame>().unwrap();
    m.add_class::<PyFrames>().unwrap();
    m.add_class::<PyGameStart>().unwrap();
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
