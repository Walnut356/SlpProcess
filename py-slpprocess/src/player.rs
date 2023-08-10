use std::sync::{Arc, RwLock};

use polars::prelude::DataFrame;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3_polars::PyDataFrame;
use slpprocess::player::Player;

use crate::game::PyFrames;

// #[derive(Clone, Debug)]
// #[pyclass(name = "Players")]
// pub struct PyPlayers {
//     pub players: [PyPlayer; 2],
// }

// impl PyPlayers {
//     pub fn new(players: [PyPlayer; 2]) -> Self {
//         Self { players }
//     }
//     pub fn get(&self, ind: usize) -> &PyPlayer {
//         &self.players[ind]
//     }
// }

// #[pymethods]
// impl PyPlayers {
//     fn __getitem__(&self, ind: usize) -> PyResult<PyPlayer> {
//         Ok(self.get(ind).clone())
//     }
// }

#[derive(Clone, Debug)]
#[pyclass(name = "Player")]
pub struct PyPlayer {
    pub player: Arc<RwLock<Player>>,
    #[pyo3(get)]
    pub frames: PyFrames,
    #[pyo3(get)]
    pub nana_frames: Option<PyFrames>,
}

impl PyPlayer {
    pub fn new(player: Arc<RwLock<Player>>) -> Self {
        let frames = PyFrames {
            pre: PyDataFrame(player.read().unwrap().frames.pre.clone()),
            post: PyDataFrame(player.read().unwrap().frames.post.clone()),
        };

        let nana_frames = {
            match player.read().unwrap().nana_frames.clone() {
                Some(nana_frames) => Some(PyFrames {
                    pre: PyDataFrame(nana_frames.pre),
                    post: PyDataFrame(nana_frames.post),
                }),
                None => None,
            }
        };
        PyPlayer {
            player,
            frames,
            nana_frames,
        }
    }
}

#[pymethods]
impl PyPlayer {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Player{{\nPort: {:?},\n}}",
            self.player.read().unwrap().port
        ))
    }

    #[getter]
    fn get_character(&self) -> PyResult<u8> {
        Ok(self
            .player
            .as_ref()
            .read()
            .unwrap()
            .character
            .try_into_css()
            .unwrap())
    }
    #[getter]
    fn get_costume(&self) -> PyResult<u8> {
        Ok(self.player.as_ref().read().unwrap().costume)
    }
    #[getter]
    fn get_port(&self) -> PyResult<u8> {
        Ok(self.player.as_ref().read().unwrap().port.into())
    }
    #[getter]
    fn get_connect_code(&self) -> PyResult<Option<String>> {
        Ok(self.player.as_ref().read().unwrap().connect_code.clone())
    }
    #[getter]
    fn get_display_name(&self) -> PyResult<Option<String>> {
        Ok(self.player.as_ref().read().unwrap().display_name.clone())
    }
    #[getter]
    fn get_is_winner(&self) -> PyResult<Option<bool>> {
        Ok(self.player.as_ref().read().unwrap().is_winner)
    }
}
