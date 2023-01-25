use pyo3::prelude::*;

use crate::{linalg::vec::vec3 as cvec3, sim, Vec3};

#[allow(dead_code)]
#[derive(Clone, Copy, FromPyObject, Debug, Default)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl From<Vector3> for cvec3 {
    fn from(value: Vector3) -> Self {
        Self {
            data: [value.x, value.y, value.z],
        }
    }
}

impl From<Vector3> for Vec3 {
    fn from(value: Vector3) -> Self {
        Self([value.x, value.y, value.z])
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, FromPyObject, Debug, Default)]
pub struct FieldBoostPad {
    pub location: Vector3,
    pub is_full_boost: bool,
}

impl From<&FieldBoostPad> for crate::sim::boost_pad::BoostPad {
    fn from(pad: &FieldBoostPad) -> Self {
        Self {
            position: pad.location.into(),
            type_: pad.is_full_boost.into(),
            state: crate::sim::boost_pad::BoostPadState::Available,
            timer: 0.,
            actor_id: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, FromPyObject, Debug, Default)]
pub struct FieldGoalInfo {
    pub team_num: u8,
    pub location: Vector3,
    pub direction: Vector3,
    pub width: f32,
    pub height: f32,
}

impl From<&FieldGoalInfo> for crate::sim::goal::Goal {
    fn from(goal: &FieldGoalInfo) -> Self {
        Self {
            team: goal.team_num,
            position: goal.location.into(),
            direction: goal.direction.into(),
            width: goal.width,
            height: goal.height,
            state: crate::sim::goal::GoalState::Unknown,
            actor_id: 0,
        }
    }
}

#[derive(Clone, FromPyObject, Debug, Default)]
pub struct FieldInfoPacket {
    num_boosts: usize,
    boost_pads: Vec<FieldBoostPad>,
    num_goals: usize,
    goals: Vec<FieldGoalInfo>,
}

impl FieldInfoPacket {
    pub fn cpads(&self) -> Vec<sim::boost_pad::BoostPad> {
        self.boost_pads[..self.num_boosts]
            .iter()
            .map(Into::into)
            .collect()
    }

    pub fn cgoals(&self) -> Vec<sim::goal::Goal> {
        self.goals[..self.num_goals]
            .iter()
            .map(Into::into)
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Default, FromPyObject)]
pub struct Hitbox {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, Default, FromPyObject)]
pub struct Rotator {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

// #[derive(Clone, Copy, Debug, Default, FromPyObject)]
// pub struct Sphere {
//     pub diameter: f32,
// }

// #[derive(Clone, Copy, Debug, Default, FromPyObject)]
// pub struct Cylinder {
//     pub diameter: f32,
//     pub height: f32,
// }

// #[derive(Clone, Copy, Debug, Default, FromPyObject)]
// pub struct CollisionShape {
//     #[pyo3(attribute("type"))]
//     shape_type: usize,
//     #[pyo3(attribute("box"))]
//     box_: Hitbox,
//     sphere: Sphere,
//     cylinder: Cylinder,
// }

#[derive(Clone, Copy, Debug, Default, FromPyObject)]
pub struct Physics {
    pub location: Vector3,
    pub velocity: Vector3,
    pub angular_velocity: Vector3,
    pub rotation: Rotator,
}

#[derive(Clone, Copy, Debug, Default, FromPyObject)]
pub struct GameBall {
    pub physics: Physics,
    // pub collision_shape: CollisionShape,
}

#[derive(Clone, Copy, Debug, Default, FromPyObject)]
pub struct GameInfo {
    pub seconds_elapsed: f32,
    pub game_time_remaining: f32,
    pub world_gravity_z: f32,
    pub is_match_ended: bool,
    pub is_round_active: bool,
    pub is_kickoff_pause: bool,
}

#[derive(Clone, Copy, Debug, Default, FromPyObject)]
pub struct Car {
    pub physics: Physics,
    pub hitbox: Hitbox,
    pub hitbox_offset: Vector3,
    pub boost: u8,
    pub jumped: bool,
    pub double_jumped: bool,
    pub is_demolished: bool,
    pub has_wheel_contact: bool,
}

#[derive(Clone, Debug, Default, FromPyObject)]
pub struct GameTickPacket {
    pub game_info: GameInfo,
    pub game_ball: GameBall,
    pub game_cars: Vec<Car>,
    pub num_cars: usize,
}
