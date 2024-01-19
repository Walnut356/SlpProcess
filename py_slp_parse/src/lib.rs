pub mod frames;
pub mod game;
pub mod player;

use game::PyGame;
use pyo3::prelude::*;
use slp_parse::Game;

#[pyfunction]
pub fn parse(path: String) -> Vec<PyGame> {
    let vals: Vec<Game> = slp_parse::parse(&path, true);

    vals.into_iter().map(PyGame::new).collect()
}

#[pymodule]
fn py_slp_parse(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<game::PyGame>().unwrap();
    m.add_class::<frames::PyFrames>().unwrap();
    m.add_class::<frames::PyPre>().unwrap();
    m.add_class::<frames::PyPost>().unwrap();
    m.add_class::<player::PyPlayer>().unwrap();
    m.add_class::<player::PyStats>().unwrap();
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
