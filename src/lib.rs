mod pytypes;

use autocxx::prelude::*;
use pyo3::{prelude::*, wrap_pyfunction, wrap_pymodule};
use pytypes::*;

include_cpp! {
    #include "rlutilities.h"
    name!(base)
    safety!(unsafe_ffi)
    generate!("rlu::initialize")
}

include_cpp! {
    #include "simulation/game.h"
    name!(sim_game)
    safety!(unsafe_ffi)
    generate!("Game")
}

include_cpp! {
    #include "simulation/ball.h"
    name!(sim_ball)
    safety!(unsafe_ffi)
    generate!("BallShape")
    generate!("Ball")
}

macro_rules! pynamedmodule {
    (doc: $doc:literal, name: $name:tt, funcs: [$($func_name:path),*], classes: [$($class_name:ident),*], submodules: [$($submodule_name:ident),*]) => {
        #[doc = $doc]
        #[pymodule]
        #[allow(redundant_semicolons)]
        fn $name(_py: Python, m: &PyModule) -> PyResult<()> {
            $(m.add_function(wrap_pyfunction!($func_name, m)?)?);*;
            $(m.add_class::<$class_name>()?);*;
            $(m.add_wrapped(wrap_pymodule!($submodule_name))?);*;
            Ok(())
        }
    };
}

#[pyclass]
#[derive(Debug, Default)]
struct Game {
    field_info: FieldInfoPacket,
}

#[pymethods]
impl Game {
    #[new]
    fn new() -> Self {
        Game::default()
    }

    fn read_field_info(&mut self, field_info: FieldInfoPacket) {
        self.field_info = field_info;
    }

    #[staticmethod]
    fn set_mode(mode: String) {
        sim_game::Game::set_mode(mode);
    }
}

#[pyclass]
struct Ball {}

pynamedmodule! {
    doc: "",
    name: simulation,
    funcs: [],
    classes: [Game, Ball],
    submodules: []
}

pynamedmodule! {
    doc: "RLUtilities bindings for Python 3.7+",
    name: rlutilities,
    funcs: [initialize],
    classes: [],
    submodules: [simulation]
}

#[pyfunction]
fn initialize(asset_dir: String) {
    base::rlu::initialize(asset_dir);
}
