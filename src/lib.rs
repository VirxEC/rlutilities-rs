mod ctypes;

pub use ctypes::linear_algebra::{mat::mat3 as cmat3, vec::vec3 as cvec3};
pub use ctypes::*;

use ctypes::{
    linear_algebra::math,
    simulation::{
        boost_pad::{BoostPadState, BoostPadType},
        car::{Car, CarBody, CarState},
        input::Input,
    },
};

impl From<[[f32; 3]; 3]> for cmat3 {
    #[inline]
    fn from(value: [[f32; 3]; 3]) -> Self {
        let mut data = [0.0; 9];

        for (i, row) in value.into_iter().enumerate() {
            for (j, val) in row.into_iter().enumerate() {
                data[i + 3 * j] = val;
            }
        }

        Self { data }
    }
}

impl cmat3 {
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> f32 {
        self.data[i + 3 * j]
    }

    #[inline]
    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut f32 {
        &mut self.data[i + 3 * j]
    }
}

impl std::ops::Mul<f32> for cmat3 {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self {
        for i in &mut self.data {
            *i *= rhs;
        }

        self
    }
}

impl std::ops::Mul<cmat3> for f32 {
    type Output = cmat3;

    #[inline]
    fn mul(self, rhs: cmat3) -> cmat3 {
        rhs * self
    }
}

impl Copy for CarBody {}
impl Copy for CarState {}

impl Car {
    pub const M: f32 = 180.;
    pub const V_MAX: f32 = 2300.;
    pub const W_MAX: f32 = 5.5;
}

impl Default for Car {
    fn default() -> Self {
        let i = Self::M * cmat3::from([[751., 0., 0.], [0., 1334., 0.], [0., 0., 1836.]]);
        Self {
            position: cvec3::default(),
            velocity: cvec3::default(),
            angular_velocity: cvec3::default(),
            orientation: math::eye(),
            supersonic: false,
            jumped: false,
            double_jumped: false,
            on_ground: false,
            demolished: false,
            boost: 0,
            jump_timer: -1.,
            dodge_timer: -1.,
            boost_timer: 0.,
            enable_jump_acceleration: false,
            dodge_torque: cvec3::default(),
            frame: 0,
            time: 0.,
            body: simulation::car::CarBody::Octane,
            state: simulation::car::CarState::OnGround,
            hitbox_widths: cvec3 {
                data: [59.003_69, 42.099_705, 18.079_536],
            },
            hitbox_offset: cvec3 {
                data: [13.975_66, 0., 20.754_988],
            },
            team: 0,
            id: 0,
            controls: Input::default(),
            I: i,
            invI: math::inv(&i),
        }
    }
}

impl Copy for BoostPadType {}
impl Copy for BoostPadState {}

impl From<bool> for BoostPadType {
    #[inline]
    fn from(value: bool) -> Self {
        if value {
            Self::Full
        } else {
            Self::Partial
        }
    }
}

impl From<bool> for BoostPadState {
    #[inline]
    fn from(value: bool) -> Self {
        if value {
            Self::Available
        } else {
            Self::Unavailable
        }
    }
}
