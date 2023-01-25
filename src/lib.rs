mod ctypes;
mod pytypes;

pub use ctypes::{linear_algebra as linalg, mechanics as mech, rlu, simulation as sim};
pub use linalg::vec::vec3 as cvec3;
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

#[pyclass]
#[repr(transparent)]
struct Game(sim::game::Game);

impl Default for Game {
    fn default() -> Self {
        Self(sim::game::Game {
            time: -1.,
            time_delta: 0.,
            time_remaining: -1.,
            gravity: cvec3 {
                data: [0., 0., -650.],
            },
            state: sim::game::GameState::Inactive,
            ball: sim::ball::Ball::default(),
            pads: sim::game::new_boostpad_vec(),
            goals: sim::game::new_goal_vec(),
        })
    }
}

#[pymethods]
impl Game {
    #[new]
    fn __new__() -> Self {
        Self::default()
    }

    #[staticmethod]
    fn set_mode(mode: String) {
        sim::game::set_mode(mode);
    }

    fn read_field_info(&mut self, field_info: FieldInfoPacket) {
        for (cpad, new_pad) in self.0.pads.pin_mut().iter_mut().zip(field_info.cpads()) {
            *cpad.get_mut() = new_pad;
        }

        for (cgoal, new_goal) in self.0.goals.pin_mut().iter_mut().zip(field_info.cgoals()) {
            *cgoal.get_mut() = new_goal;
        }
    }

    fn read_packet(&mut self, packet: GameTickPacket) {
        self.0.time_delta = packet.game_info.seconds_elapsed - self.0.time;
        self.0.time = packet.game_info.seconds_elapsed;
        self.0.time_remaining = packet.game_info.game_time_remaining;
        self.0.gravity.data[2] = packet.game_info.world_gravity_z;

        self.0.state = if packet.game_info.is_match_ended {
            sim::game::GameState::Ended
        } else if packet.game_info.is_round_active {
            if packet.game_info.is_kickoff_pause {
                sim::game::GameState::Kickoff
            } else {
                sim::game::GameState::Active
            }
        } else {
            sim::game::GameState::Inactive
        };

        self.0.ball.time = packet.game_info.seconds_elapsed;
        self.0.ball.position = packet.game_ball.physics.location.into();
        self.0.ball.velocity = packet.game_ball.physics.velocity.into();
        self.0.ball.angular_velocity = packet.game_ball.physics.angular_velocity.into();
    }

    #[getter(ball)]
    fn get_ball(&self) -> Ball {
        self.0.ball.clone().into()
    }

    #[setter(ball)]
    fn set_ball(&mut self, ball: Ball) {
        self.0.ball = ball.into();
    }
}

#[pyclass(get_all, set_all)]
#[derive(Clone, Debug)]
struct Ball {
    time: f32,
    position: Vec3,
    velocity: Vec3,
    angular_velocity: Vec3,
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            time: 0.,
            position: Vec3([0., 0., 110.]),
            velocity: Vec3::default(),
            angular_velocity: Vec3::default(),
        }
    }
}

impl From<sim::ball::Ball> for Ball {
    #[inline]
    fn from(ball: sim::ball::Ball) -> Self {
        Self {
            time: ball.time,
            position: ball.position.into(),
            velocity: ball.velocity.into(),
            angular_velocity: ball.angular_velocity.into(),
        }
    }
}

impl From<Ball> for sim::ball::Ball {
    #[inline]
    fn from(ball: Ball) -> Self {
        Self {
            time: ball.time,
            position: ball.position.into(),
            velocity: ball.velocity.into(),
            angular_velocity: ball.angular_velocity.into(),
        }
    }
}

#[pymethods]
impl Ball {
    const NAMES: [&str; 4] = ["time", "position", "velocity", "angular_velocity"];

    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    fn __new__(args: &PyTuple, kwargs: Option<&PyAny>) -> Self {
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
            Ball {
                time: time.unwrap_or_default(),
                position: vec[0].unwrap_or_default(),
                velocity: vec[1].unwrap_or_default(),
                angular_velocity: vec[2].unwrap_or_default(),
            }
        }
    }

    fn step(&mut self, dt: f32) {
        // this code might look like a crime against humanity
        // and I won't deny that but the performance impact is negligible
        // it's well optimized by the compiler and makes syntax cleaner elsewhere
        let mut ball: sim::ball::Ball = self.clone().into();
        ball.step(dt);
        *self = ball.into();
    }

    fn __str__(&self) -> String {
        format!(
            "Ball: time={}, position={}, velocity={}, angular_velocity={}",
            self.time,
            self.position.__str__(),
            self.velocity.__str__(),
            self.angular_velocity.__str__()
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "Ball(time={}, position={}, velocity={}, angular_velocity={})",
            self.time,
            self.position.__repr__(),
            self.velocity.__repr__(),
            self.angular_velocity.__repr__()
        )
    }
}

#[pyclass]
#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
#[pyo3(name = "vec3")]
#[repr(transparent)]
struct Vec3([f32; 3]);

impl From<cvec3> for Vec3 {
    #[inline]
    fn from(value: cvec3) -> Self {
        Self(value.data)
    }
}

impl From<Vec3> for cvec3 {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self { data: value.0 }
    }
}

#[pymethods]
impl Vec3 {
    const NAMES: [&str; 3] = ["x", "y", "z"];

    #[new]
    #[pyo3(signature = (*args, **kwargs))]
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
