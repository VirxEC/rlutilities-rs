mod ctypes;
mod pytypes;

use autocxx::prelude::*;
use ctypes::{rlu, simulation as sim};
use pyo3::{exceptions::PyIndexError, prelude::*, types::PyTuple, wrap_pyfunction, wrap_pymodule};
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
    _inner: Pin<Box<sim::game::Game>>,
}

#[pymethods]
impl Game {
    #[new]
    fn new() -> Self {
        Self {
            _inner: sim::game::Game::new().within_box(),
        }
    }

    #[staticmethod]
    fn set_mode(mode: String) {
        sim::game::Game::set_mode(mode);
    }

    fn read_field_info(&mut self, field_info: FieldInfoPacket) {
        self._inner.as_mut().resize_pads(field_info.num_boosts());
        for (i, pad) in field_info.pads().iter().enumerate() {
            self._inner.as_mut().reset_pad(
                c_int(i as i32),
                pad.location.x,
                pad.location.y,
                pad.location.z,
                pad.is_full_boost,
            );
        }

        self._inner.as_mut().resize_goals(field_info.num_goals());
        for (i, goal) in field_info.goals().iter().enumerate() {
            self._inner.as_mut().reset_goal(
                c_int(i as i32),
                goal.location.x,
                goal.location.y,
                goal.location.z,
                goal.direction.x,
                goal.direction.y,
                goal.direction.z,
                goal.width,
                goal.height,
                c_int(goal.team_num as i32),
            );
        }
    }

    fn read_packet(&mut self, packet: GameTickPacket) {
        self._inner.as_mut().set_game_info(
            packet.game_info.seconds_elapsed,
            packet.game_info.game_time_remaining,
            packet.game_info.world_gravity_z,
            packet.game_info.is_match_ended,
            packet.game_info.is_round_active,
            packet.game_info.is_kickoff_pause,
        );
    }
}

#[pyclass]
struct Ball {}

#[pyclass]
#[derive(Clone, Copy)]
#[pyo3(name = "vec3")]
struct Vec3([f32; 3]);

#[pymethods]
impl Vec3 {
    const VEC3_ITEMS: usize = 3;
    const NAMES: [&str; 3] = ["x", "y", "z"];

    #[new]
    #[args(args = "*", kwargs = "**")]
    fn new(args: &PyTuple, kwargs: Option<&PyAny>) -> Self {
        if let Ok(args) = args.get_item(0).and_then(|x| x.extract::<Vec3>()) {
            return args;
        }

        let mut vec = [None; Self::VEC3_ITEMS];

        if let Ok(args) = args.get_item(0).and_then(|x| x.extract::<Vec<f32>>()) {
            vec.iter_mut()
                .zip(args.into_iter())
                .for_each(|(a, b)| *a = Some(b));
        } else if let Ok(args) = args.extract::<Vec<f32>>() {
            vec.iter_mut()
                .zip(args.into_iter())
                .for_each(|(a, b)| *a = Some(b));
        } else {
            for (a, b) in vec.iter_mut().zip(args.into_iter()) {
                if let Ok(x) = b.extract() {
                    *a = Some(x);
                }
            }
        }

        if let Some(kwargs) = kwargs {
            for (a, b) in vec.iter_mut().zip(Self::NAMES.into_iter()) {
                if let Ok(x) = kwargs.get_item(b).and_then(|x| x.extract()) {
                    *a = Some(x);
                }
            }
        }

        Self([
            vec[0].unwrap_or_default(),
            vec[1].unwrap_or_default(),
            vec[2].unwrap_or_default(),
        ])
    }

    fn __getitem__(&self, index: usize) -> PyResult<f32> {
        if index >= Self::VEC3_ITEMS {
            Err(PyIndexError::new_err("index out of range"))
        } else {
            Ok(self.0[index])
        }
    }

    fn __setitem__(&mut self, index: usize, value: f32) -> PyResult<()> {
        if index >= Self::VEC3_ITEMS {
            Err(PyIndexError::new_err("index out of range"))
        } else {
            self.0[index] = value;
            Ok(())
        }
    }

    #[getter(x)]
    fn get_x(&self) -> f32 {
        self.0[0]
    }

    #[setter(x)]
    fn set_x(&mut self, x: f32) {
        self.0[0] = x;
    }

    #[getter(y)]
    fn get_y(&self) -> f32 {
        self.0[1]
    }

    #[setter(y)]
    fn set_y(&mut self, y: f32) {
        self.0[1] = y;
    }

    #[getter(z)]
    fn get_z(&self) -> f32 {
        self.0[2]
    }

    #[setter(z)]
    fn set_z(&mut self, z: f32) {
        self.0[2] = z;
    }

    fn __str__(&self) -> String {
        format!("({:.2}, {:.2}, {:.2})", self.0[0], self.0[1], self.0[2])
    }

    fn __repr__(&self) -> String {
        format!("vec3(x={}, y={}, z={})", self.0[0], self.0[1], self.0[2])
    }
}

pynamedmodule! {
    doc: "",
    name: linear_algebra,
    funcs: [],
    classes: [Vec3],
    submodules: []
}

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
    submodules: [simulation, linear_algebra]
}

#[pyfunction]
fn initialize(asset_dir: String) {
    rlu::initialize(asset_dir);
}
