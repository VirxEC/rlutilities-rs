use autocxx::c_int;
use pyo3::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Copy, FromPyObject, Debug, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vector3> for [f32; 3] {
    fn from(value: Vector3) -> Self {
        [value.x, value.y, value.z]
    }
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

#[derive(Clone, Copy, Debug, Default, FromPyObject)]
pub struct GameTickPacket {
    pub game_info: GameInfo,
    pub game_ball: GameBall,
    pub num_cars: usize,
}
