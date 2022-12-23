use autocxx::c_int;
use pyo3::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Copy, FromPyObject, Debug, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
#[derive(Clone, Copy, FromPyObject, Debug, Default)]
pub struct BoostPad {
    pub location: Vector3,
    pub is_full_boost: bool,
}

#[allow(dead_code)]
#[derive(Clone, Copy, FromPyObject, Debug, Default)]
pub struct GoalInfo {
    pub team_num: u8,
    pub location: Vector3,
    pub direction: Vector3,
    pub width: f32,
    pub height: f32,
}

#[allow(dead_code)]
#[derive(Clone, FromPyObject, Debug, Default)]
pub struct FieldInfoPacket {
    num_boosts: usize,
    boost_pads: Vec<BoostPad>,
    num_goals: usize,
    goals: Vec<GoalInfo>,
}

impl FieldInfoPacket {
    pub fn num_boosts(&self) -> autocxx::c_int {
        c_int(self.num_boosts as i32)
    }

    pub fn pads(&self) -> &[BoostPad] {
        &self.boost_pads[..self.num_boosts]
    }

    pub fn num_goals(&self) -> autocxx::c_int {
        c_int(self.num_goals as i32)
    }

    pub fn goals(&self) -> &[GoalInfo] {
        &self.goals[..self.num_goals]
    }
}
