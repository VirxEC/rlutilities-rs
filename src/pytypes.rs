use pyo3::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Copy, FromPyObject, Debug, Default)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[allow(dead_code)]
#[derive(Clone, Copy, FromPyObject, Debug, Default)]
pub struct BoostPad {
    location: Vector3,
    is_full_boost: bool,
}

#[allow(dead_code)]
#[derive(Clone, Copy, FromPyObject, Debug, Default)]
pub struct GoalInfo {
    team_num: u8,
    location: Vector3,
    direction: Vector3,
    width: f32,
    height: f32,
}

#[allow(dead_code)]
#[derive(Clone, FromPyObject, Debug, Default)]
pub struct FieldInfoPacket {
    num_boosts: u8,
    boost_pads: Vec<BoostPad>,
    num_goals: u16,
    goals: Vec<GoalInfo>,
}
