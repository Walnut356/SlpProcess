use std::sync::Arc;

use pyo3::prelude::*;
use slpprocess::player::Player;

use crate::frames::{PyFrames, PyPre, PyPost};

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
}

impl PyPlayer {
    pub fn new(player: Arc<Player>) -> Self {
        let frames = PyFrames {
            pre: PyPre {
                pre: player.frames.pre.clone(),
            },
            post: PyPost {
                post: player.frames.post.clone(),
            },
        };

        let nana_frames = {
            player.nana_frames.as_ref().map(|nana_frames| PyFrames {
                pre: PyPre {
                    pre: nana_frames.pre.clone(),
                },
                post: PyPost {
                    post: nana_frames.post.clone(),
                },
            })
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
        Ok(format!("Player{{\nPort: {:?},\n}}", self.player.port))
    }

    #[getter]
    fn get_character(&self) -> PyResult<u8> {
        Ok(self.player.as_ref().character.try_into_css().unwrap())
    }
    #[getter]
    fn get_costume(&self) -> PyResult<u8> {
        Ok(self.player.costume)
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
