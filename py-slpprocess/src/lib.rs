pub mod game;

use game::PyGame;
use pyo3::prelude::*;
use slpprocess::Game;

#[pyfunction]
pub fn parse(path: String) -> Vec<PyGame> {
    let vals: Vec<Game> = slpprocess::parse(&path);

    vals.into_iter().map(PyGame::new).collect()
}

#[pymodule]
fn slp_process(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<game::PyGame>().unwrap();
    m.add_class::<game::PyFrames>().unwrap();
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
