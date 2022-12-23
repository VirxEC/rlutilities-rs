mod ctypes;
mod pytypes;

use autocxx::prelude::*;
use ctypes::{linear_algebra as lin, rlu, simulation as sim};
use pyo3::{prelude::*, wrap_pyfunction, wrap_pymodule};
use pytypes::*;
use std::pin::Pin;

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

#[pyclass(unsendable)]
struct Game {
    game: Pin<Box<sim::game::Game>>,
}

#[pymethods]
impl Game {
    #[new]
    fn new() -> Self {
        Self {
            game: sim::game::Game::new().within_box(),
        }
    }

    fn read_field_info(&mut self, field_info: FieldInfoPacket) {
        self.game.as_mut().resize_pads(field_info.num_boosts());
        for (i, pad) in field_info.pads().iter().enumerate() {
            self.game.as_mut().reset_pad(c_int(i as i32), pad.location.x, pad.location.y, pad.location.z, pad.is_full_boost);
        }

        self.game.as_mut().resize_goals(field_info.num_goals());
        for (i, goal) in field_info.goals().iter().enumerate() {
            self.game.as_mut().reset_goal(c_int(i as i32), goal.location.x, goal.location.y, goal.location.z, goal.direction.x, goal.direction.y, goal.direction.z, goal.width, goal.height, c_int(goal.team_num as i32));
        }
    }

    #[staticmethod]
    fn set_mode(mode: String) {
        sim::game::Game::set_mode(mode);
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
    rlu::initialize(asset_dir);
}
