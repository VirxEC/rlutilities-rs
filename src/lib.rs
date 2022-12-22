use autocxx::prelude::*;
use pyo3::{prelude::*, wrap_pyfunction, wrap_pymodule};

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
    #include "misc/config.h"
    name!(misc_config)
    safety!(unsafe)
    generate!("ASSET_DIR")
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
        #[allow(non_snake_case)]
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
struct Game {}

#[pymethods]
impl Game {
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
    funcs: [],
    classes: [],
    submodules: [simulation]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main() {
        use std::str::from_utf8;

        // Converts to utf8 and removes null terminator
        let asset_dir = from_utf8(misc_config::ASSET_DIR)
            .unwrap()
            .trim_end_matches(char::from(0));

        base::rlu::initialize(asset_dir);

        sim_game::Game::set_mode("soccar");

        println!("Hello, world!");
    }
}
