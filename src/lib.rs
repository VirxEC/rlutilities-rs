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
struct Game(Pin<Box<sim::game::Game>>);

#[pymethods]
impl Game {
    #[new]
    fn new() -> Self {
        Self(sim::game::Game::new().within_box())
    }

    #[staticmethod]
    fn set_mode(mode: String) {
        sim::game::Game::set_mode(mode);
    }

    fn read_field_info(&mut self, field_info: FieldInfoPacket) {
        self.0.as_mut().resize_pads(field_info.num_boosts());
        for (i, pad) in field_info.pads().iter().enumerate() {
            self.0
                .as_mut()
                .reset_pad_2(i as i32, pad.location.into(), pad.is_full_boost);
        }

        self.0.as_mut().resize_goals(field_info.num_goals());
        for (i, goal) in field_info.goals().iter().enumerate() {
            self.0.as_mut().reset_goal_2(
                i as i32,
                goal.location.into(),
                goal.direction.into(),
                goal.width,
                goal.height,
                i32::from(goal.team_num),
            );
        }
    }

    fn read_packet(&mut self, packet: GameTickPacket) {
        self.0.as_mut().set_game_info(
            packet.game_info.seconds_elapsed,
            packet.game_info.game_time_remaining,
            packet.game_info.world_gravity_z,
            packet.game_info.is_match_ended,
            packet.game_info.is_round_active,
            packet.game_info.is_kickoff_pause,
        );
    }
}

impl From<GameBall> for Ball {
    fn from(pball: GameBall) -> Self {
        let mut sim_ball = sim::ball::Ball::new().within_box();

        sim_ball.as_mut().update_2(
            pball.physics.location.into(),
            pball.physics.velocity.into(),
            pball.physics.angular_velocity.into(),
        );

        Self(sim_ball)
    }
}

#[pyclass(unsendable)]
struct Ball(Pin<Box<sim::ball::Ball>>);

impl Default for Ball {
    fn default() -> Self {
        Self(sim::ball::Ball::new().within_box())
    }
}

#[pymethods]
impl Ball {
    #[new]
    fn new(packet_ball: Option<GameBall>) -> Self {
        packet_ball.map(Into::into).unwrap_or_default()
    }

    fn step(&mut self, dt: f32) {
        self.0.as_mut().step(dt);
    }

    #[getter(position)]
    fn get_position(&self) -> Vec3 {
        Vec3(self.0.get_position_2())
    }

    fn __str__(&self) -> String {
        format!("Ball: position={}", self.get_position().__str__())
    }

    fn repr(&self) -> String {
        format!("Ball(position={})", self.get_position().__repr__())
    }
}

#[pyclass]
#[derive(Clone, Copy)]
#[pyo3(name = "vec3")]
struct Vec3([f32; 3]);

impl Into<[f32; 3]> for Vec3 {
    fn into(self) -> [f32; 3] {
        self.0
    }
}

#[pymethods]
impl Vec3 {
    const VEC3_ITEMS: usize = 3;
    const NAMES: [&str; 3] = ["x", "y", "z"];

    #[new]
    #[args(args = "*", kwargs = "**")]
    fn new(args: &PyTuple, kwargs: Option<&PyAny>) -> Self {
        if let Ok(args) = args.get_item(0).and_then(PyAny::extract) {
            return args;
        }

        let mut vec = [None; Self::VEC3_ITEMS];

        if let Ok(args) = args.get_item(0).and_then(PyAny::extract::<Vec<f32>>) {
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
                if let Ok(x) = kwargs.get_item(b).and_then(PyAny::extract) {
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
