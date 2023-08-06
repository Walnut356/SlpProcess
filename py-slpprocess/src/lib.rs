use std::path::Path;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use slpprocess::Game;

#[repr(transparent)]
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

    pub fn get_port_frames(&self, port: u8) -> PyResult<PyDataFrame> {
        let normalized = (port - 1).try_into().unwrap();
        for player in self.game.players.iter() {
            if player.port == normalized {
                return Ok(PyDataFrame(player.frames.pre.clone()));
            }
        }
        PyResult::Err(PyValueError::new_err(format!("No player with port {port}")))
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
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
