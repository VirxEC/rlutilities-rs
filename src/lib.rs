mod ctypes;
mod pytypes;

use std::fmt;

use autocxx::prelude::*;
pub use ctypes::{linear_algebra as linalg, mechanics as mech, rlu, simulation as sim};
use pyo3::{
    exceptions::PyIndexError, prelude::*, pyclass::CompareOp, types::PyTuple, wrap_pyfunction,
    wrap_pymodule,
};
use pytypes::*;

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
struct Game(UniquePtr<sim::game::Game>);

#[pymethods]
impl Game {
    #[new]
    fn new() -> Self {
        Self(sim::game::Game::new().within_unique_ptr())
    }

    #[staticmethod]
    fn set_mode(mode: String) {
        sim::game::Game::set_mode(mode);
    }

    fn read_field_info(&mut self, field_info: FieldInfoPacket) {
        self.0.pin_mut().resize_pads(field_info.num_boosts());
        for (i, pad) in field_info.pads().iter().enumerate() {
            self.0
                .pin_mut()
                .reset_pad_2(i as i32, pad.location.into(), pad.is_full_boost);
        }

        self.0.pin_mut().resize_goals(field_info.num_goals());
        for (i, goal) in field_info.goals().iter().enumerate() {
            self.0.pin_mut().reset_goal_2(
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
        self.0.pin_mut().set_game_info(
            packet.game_info.seconds_elapsed,
            packet.game_info.game_time_remaining,
            packet.game_info.world_gravity_z,
            packet.game_info.is_match_ended,
            packet.game_info.is_round_active,
            packet.game_info.is_kickoff_pause,
        );

        let mut ball = self.get_ball();
        ball.set_time(packet.game_info.seconds_elapsed);
        ball.set_position_g(packet.game_ball.physics.location);
        ball.set_velocity_g(packet.game_ball.physics.velocity);
        ball.set_angular_velocity_g(packet.game_ball.physics.angular_velocity);
        self.0.pin_mut().set_ball(ball.0);
    }

    #[getter(ball)]
    fn get_ball(&self) -> Ball {
        Ball(self.0.get_ball())
    }

    #[setter(ball)]
    fn set_ball(&mut self, ball: Ball) {
        self.0.pin_mut().set_ball(ball.0);
    }
}

#[pyclass(unsendable)]
struct Ball(UniquePtr<sim::ball::Ball>);

impl fmt::Debug for Ball {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ball")
            .field("time", &self.get_time())
            .field("position", &self.get_position())
            .field("velocity", &self.get_velocity())
            .field("angular_velocity", &self.get_angular_velocity())
            .finish()
    }
}

impl Clone for Ball {
    fn clone(&self) -> Self {
        let mut ball = sim::ball::Ball::new().within_unique_ptr();
        ball.pin_mut().update_from_ball(&self.0);
        Self(ball)
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self(sim::ball::Ball::new().within_unique_ptr())
    }
}

impl Ball {
    fn set_position_g<V: Into<linalg::vec::vec3>>(&mut self, pos: V) {
        self.0.pin_mut().set_position_2(pos.into());
    }

    fn set_velocity_g<V: Into<linalg::vec::vec3>>(&mut self, vel: V) {
        self.0.pin_mut().set_velocity_2(vel.into());
    }

    fn set_angular_velocity_g<V: Into<linalg::vec::vec3>>(&mut self, ang_vel: V) {
        self.0.pin_mut().set_angular_velocity_2(ang_vel.into());
    }
}

#[pymethods]
impl Ball {
    const NAMES: [&str; 4] = ["time", "position", "velocity", "angular_velocity"];

    #[new]
    #[args(args = "*", kwargs = "**")]
    fn new(args: &PyTuple, kwargs: Option<&PyAny>) -> Self {
        if let Ok(args) = args.get_item(0).and_then(PyAny::extract) {
            return args;
        }

        let mut vec = [None; Self::NAMES.len() - 1];

        let mut time = args.get_item(0).and_then(PyAny::extract).ok();

        if let Ok(args) = args.extract::<Vec<Vec3>>() {
            vec.iter_mut()
                .zip(args.into_iter().skip(1))
                .for_each(|(a, b)| *a = Some(b));
        } else {
            for (a, b) in vec.iter_mut().zip(args.into_iter().skip(1)) {
                if let Ok(x) = b.extract() {
                    *a = Some(x);
                }
            }
        }

        if let Some(kwargs) = kwargs {
            if let Ok(arg) = kwargs.get_item(Self::NAMES[0]).and_then(PyAny::extract) {
                time = Some(arg);
            }

            for (a, b) in vec.iter_mut().zip(Self::NAMES.into_iter().skip(1)) {
                if let Ok(x) = kwargs.get_item(b).and_then(PyAny::extract) {
                    *a = Some(x);
                }
            }
        }

        // if there are no items in vec that are Some, then we can just return the default
        if vec.iter().all(|x| x.is_none()) {
            Self::default()
        } else {
            let mut ball = Self::default();

            if let Some(time) = time {
                ball.set_time(time);
            }

            if let Some(pos) = vec[0] {
                ball.set_position_g(pos);
            }

            if let Some(vel) = vec[1] {
                ball.set_velocity_g(vel);
            }

            if let Some(ang_vel) = vec[2] {
                ball.set_angular_velocity_g(ang_vel);
            }

            ball
        }
    }

    fn step(&mut self, dt: f32) {
        self.0.pin_mut().step(dt);
    }

    #[getter(time)]
    fn get_time(&self) -> f32 {
        self.0.get_time()
    }

    #[setter(time)]
    fn set_time(&mut self, time: f32) {
        self.0.pin_mut().set_time(time);
    }

    #[getter(position)]
    fn get_position(&self) -> Vec3 {
        Vec3(self.0.get_position_2().data)
    }

    #[setter(position)]
    fn set_position(&mut self, pos: Vec3) {
        self.set_position_g(pos);
    }

    #[getter(velocity)]
    fn get_velocity(&self) -> Vec3 {
        Vec3(self.0.get_velocity_2().data)
    }

    #[setter(velocity)]
    fn set_velocity(&mut self, vel: Vec3) {
        self.set_velocity_g(vel);
    }

    #[getter(angular_velocity)]
    fn get_angular_velocity(&self) -> Vec3 {
        Vec3(self.0.get_angular_velocity_2().data)
    }

    #[setter(angular_velocity)]
    fn set_angular_velocity(&mut self, vel: Vec3) {
        self.set_angular_velocity_g(vel);
    }

    fn __str__(&self) -> String {
        format!(
            "Ball: time={}, position={}, velocity={}, angular_velocity={}",
            self.get_time(),
            self.get_position().__str__(),
            self.get_velocity().__str__(),
            self.get_angular_velocity().__str__()
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "Ball(time={}, position={}, velocity={}, angular_velocity={})",
            self.get_time(),
            self.get_position().__repr__(),
            self.get_velocity().__repr__(),
            self.get_angular_velocity().__repr__()
        )
    }
}

#[pyclass]
#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
#[pyo3(name = "vec3")]
struct Vec3([f32; 3]);

impl From<Vec3> for linalg::vec::vec3 {
    fn from(value: Vec3) -> Self {
        Self { data: value.0 }
    }
}

#[pymethods]
impl Vec3 {
    const NAMES: [&str; 3] = ["x", "y", "z"];

    #[new]
    #[args(args = "*", kwargs = "**")]
    fn new(args: &PyTuple, kwargs: Option<&PyAny>) -> Self {
        if let Ok(args) = args.get_item(0).and_then(PyAny::extract) {
            return args;
        }

        let mut vec = [None; Self::NAMES.len()];

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
        if index >= Self::NAMES.len() {
            Err(PyIndexError::new_err("index out of range"))
        } else {
            Ok(self.0[index])
        }
    }

    fn __setitem__(&mut self, index: usize, value: f32) -> PyResult<()> {
        if index >= Self::NAMES.len() {
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

    /// Only == and != are actually supported right now
    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        if !matches!(op, CompareOp::Eq | CompareOp::Ne) {
            return false;
        };

        let Some(cmp) = self.partial_cmp(&other) else {
            return false;
        };

        op.matches(cmp)
    }
}

#[pyclass]
struct Field();

#[pyclass]
struct Drive();

// #[pymethods]
// impl Drive {
//     fn get_controls() {
//         Python::with_gil(|py| {
//             py.import("rlbot.")
//         })
//     }
// }

pynamedmodule! {
    doc: "",
    name: mechanics,
    funcs: [],
    classes: [Drive],
    submodules: []
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
    classes: [Game, Ball, Field],
    submodules: []
}

pynamedmodule! {
    doc: "RLUtilities bindings for Python 3.7+",
    name: rlutilities,
    funcs: [initialize],
    classes: [],
    submodules: [simulation, linear_algebra, mechanics]
}

#[pyfunction]
fn initialize(asset_dir: String) {
    rlu::initialize(asset_dir);
}
