use std::sync::Arc;

use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use slp_parse::prelude::*;

use crate::frames::{PyFrames, PyPost, PyPre};

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
#[pyclass(name = "Player", frozen)]
pub struct PyPlayer {
    pub player: Arc<Player>,
    #[pyo3(get)]
    pub frames: PyFrames,
    #[pyo3(get)]
    pub nana_frames: Option<PyFrames>,
    #[pyo3(get)]
    pub stats: PyStats,
}

impl PyPlayer {
    pub fn new(player: Arc<Player>) -> Self {
        let frames = PyFrames {
            pre: PyPre {
                frames: player.frames.pre.clone(),
            },
            post: PyPost {
                frames: player.frames.post.clone(),
            },
        };

        let nana_frames = {
            player.nana_frames.as_ref().map(|nana_frames| PyFrames {
                pre: PyPre {
                    frames: nana_frames.pre.clone(),
                },
                post: PyPost {
                    frames: nana_frames.post.clone(),
                },
            })
        };

        let stats = PyStats {
            stats: player.stats.clone(),
        };
        PyPlayer {
            player,
            frames,
            nana_frames,
            stats,
        }
    }
}

#[pymethods]
impl PyPlayer {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Player{{\nPort: {:?},\n}}", self.player.port))
    }

    #[getter]
    fn get_character(&self) -> PyResult<u8> {
        Ok(self.player.as_ref().character.try_as_css().unwrap())
    }
    #[getter]
    fn get_costume(&self) -> PyResult<u8> {
        Ok(self.player.costume as u8)
    }
    #[getter]
    fn get_port(&self) -> PyResult<u8> {
        Ok(self.player.port as u8)
    }
    #[getter]
    fn get_connect_code(&self) -> PyResult<Option<String>> {
        Ok(self.player.connect_code.clone())
    }
    #[getter]
    fn get_display_name(&self) -> PyResult<Option<String>> {
        Ok(self.player.display_name.clone())
    }
    #[getter]
    fn get_is_winner(&self) -> PyResult<Option<bool>> {
        Ok(self.player.is_winner)
    }
}

#[derive(Clone, Debug)]
#[pyclass(name = "Stats", frozen)]
pub struct PyStats {
    stats: Arc<Stats>,
}

#[pymethods]
impl PyStats {
    #[getter]
    fn get_input(&self) -> PyResult<PyDataFrame> {
        Ok(PyDataFrame(self.stats.input.clone()))
    }

    #[getter]
    fn get_l_cancel(&self) -> PyResult<Option<PyDataFrame>> {
        Ok(self
            .stats
            .l_cancel
            .as_ref()
            .map(|df| PyDataFrame(df.clone())))
    }
    #[getter]
    fn get_item(&self) -> PyResult<Option<PyDataFrame>> {
        Ok(self.stats.item.as_ref().map(|df| PyDataFrame(df.clone())))
    }
    #[getter]
    fn get_defense(&self) -> PyResult<Option<PyDataFrame>> {
        Ok(self
            .stats
            .defense
            .as_ref()
            .map(|df| PyDataFrame(df.clone())))
    }
    #[getter]
    fn get_tech(&self) -> PyResult<Option<PyDataFrame>> {
        Ok(self.stats.tech.as_ref().map(|df| PyDataFrame(df.clone())))
    }
}
