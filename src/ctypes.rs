pub mod rlu {
    autocxx::include_cpp! {
        #include "rlutilities.h"
        name!(base)
        safety!(unsafe)
        generate!("rlu::initialize")
    }

    pub use base::rlu::initialize;
}

pub mod linear_algebra {
    pub mod vec {
        #[cxx::bridge]
        mod linalg_vec {
            #[derive(Clone, Copy, Debug, Default)]
            struct vec3 {
                data: [f32; 3],
            }
        }

        pub use linalg_vec::vec3;
    }

    pub mod mat {
        #[cxx::bridge]
        mod linalg_mat {
            #[derive(Clone, Copy, Debug, Default)]
            struct mat3 {
                data: [f32; 9],
            }
        }

        pub use linalg_mat::mat3;
    }

    impl From<[[f32; 3]; 3]> for mat::mat3 {
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

    impl mat::mat3 {
        #[inline]
        pub fn get(&self, i: usize, j: usize) -> f32 {
            self.data[i + 3 * j]
        }

        #[inline]
        pub fn get_mut(&mut self, i: usize, j: usize) -> &mut f32 {
            &mut self.data[i + 3 * j]
        }
    }

    impl std::ops::Mul<f32> for mat::mat3 {
        type Output = Self;

        fn mul(mut self, rhs: f32) -> Self {
            for i in &mut self.data {
                *i *= rhs;
            }

            self
        }
    }

    impl std::ops::Mul<mat::mat3> for f32 {
        type Output = mat::mat3;

        #[inline]
        fn mul(self, rhs: mat::mat3) -> mat::mat3 {
            rhs * self
        }
    }

    pub mod math {
        #[cxx::bridge]
        mod linalg_math {
            unsafe extern "C++" {
                include!("linear_algebra/math.h");

                type vec3 = crate::cvec3;
                type mat3 = crate::cmat3;

                fn eye() -> mat3;
                fn inv(A: &mat3) -> mat3;
                fn euler_to_rotation(pyr: &vec3) -> mat3;
            }
        }

        pub use linalg_math::*;
    }
}

pub mod mechanics {
    pub mod drive {
        #[cxx::bridge]
        mod mech_drive {
            unsafe extern "C++" {
                include!("mechanics/drive.h");

                type vec3 = crate::cvec3;
                type Car = crate::sim::car::Car;
                type Input = crate::sim::input::Input;
                type Drive;

                #[must_use]
                fn new_drive(car: Car) -> Drive;

                fn step(self: &mut Drive, dt: f32);
            }

            #[derive(Clone, Copy, Default)]
            struct Drive {
                car: Car,
                target: vec3,
                speed: f32,
                reaction_time: f32,
                finished: bool,
                controls: Input,
            }
        }

        pub use mech_drive::Drive;
        use mech_drive::{new_drive, Car};

        impl Drive {
            pub fn new(car: Car) -> Self {
                new_drive(car)
            }
        }
    }
}

pub mod simulation {
    pub mod game {
        autocxx::include_cpp! {
            #include "simulation/game.h"
            name!(sim_game)
            safety!(unsafe)
            generate_pod!("GameState")
        }

        #[cxx::bridge]
        mod sim_game_extra {
            unsafe extern "C++" {
                include!("simulation/game.h");

                type vec3 = crate::cvec3;
                type BoostPad = crate::sim::boost_pad::BoostPad;
                type GameState = super::sim_game::GameState;
                type Ball = crate::sim::ball::Ball;
                type Goal = crate::sim::goal::Goal;
                type Car = crate::sim::car::Car;
                type Game;

                fn set_mode(gamemode: String);

                #[must_use]
                fn new_boostpad_vec() -> UniquePtr<CxxVector<BoostPad>>;

                #[must_use]
                fn new_goal_vec() -> UniquePtr<CxxVector<Goal>>;

                #[must_use]
                fn new_car_vec() -> UniquePtr<CxxVector<Car>>;
            }

            struct Game {
                time: f32,
                time_delta: f32,
                time_remaining: f32,
                gravity: vec3,
                state: GameState,
                ball: Ball,
                pads: UniquePtr<CxxVector<BoostPad>>,
                goals: UniquePtr<CxxVector<Goal>>,
                cars: UniquePtr<CxxVector<Car>>,
            }
        }

        use sim_game_extra::{new_boostpad_vec, new_car_vec, new_goal_vec, set_mode, vec3, Ball};
        pub use sim_game_extra::{Game, GameState};

        impl Game {
            pub fn set_mode(gamemode: String) {
                set_mode(gamemode)
            }
        }

        impl Default for Game {
            #[inline]
            fn default() -> Self {
                Self {
                    time: -1.,
                    time_delta: 0.,
                    time_remaining: -1.,
                    gravity: vec3 { data: [0., 0., -650.] },
                    state: GameState::Inactive,
                    ball: Ball::default(),
                    pads: new_boostpad_vec(),
                    goals: new_goal_vec(),
                    cars: new_car_vec(),
                }
            }
        }
    }

    pub mod input {
        #[cxx::bridge]
        mod sim_input {
            unsafe extern "C++" {
                include!("simulation/input.h");

                type Input;
            }

            #[derive(Clone, Copy, Default, Debug)]
            struct Input {
                pub steer: f32,
                pub roll: f32,
                pub pitch: f32,
                pub yaw: f32,
                pub throttle: f32,
                pub jump: bool,
                pub boost: bool,
                pub handbrake: bool,
                pub use_item: bool,
            }
        }

        pub use sim_input::Input;
    }

    pub mod car {
        // ignore this lint for sake of parity with RLU
        #![allow(clippy::excessive_precision)]

        autocxx::include_cpp! {
            #include "simulation/car.h"
            name!(sim_car)
            safety!(unsafe)
            generate_pod!("CarBody")
            generate_pod!("CarState")
        }

        #[cxx::bridge]
        mod sim_car_extra {
            unsafe extern "C++" {
                include!("simulation/car.h");

                type vec3 = crate::cvec3;
                type mat3 = crate::cmat3;
                type CarBody = super::sim_car::CarBody;
                type CarState = super::sim_car::CarState;
                type Input = crate::sim::input::Input;
                type Car;

                fn step(self: &mut Car, in_: Input, dt: f32);
            }

            #[derive(Clone, Copy)]
            struct Car {
                position: vec3,
                velocity: vec3,
                angular_velocity: vec3,
                orientation: mat3,
                supersonic: bool,
                jumped: bool,
                double_jumped: bool,
                on_ground: bool,
                demolished: bool,
                boost: i32,
                jump_timer: f32,
                dodge_timer: f32,
                boost_timer: f32,
                enable_jump_acceleration: bool,
                dodge_torque: vec3,
                frame: i32,
                time: f32,
                body: CarBody,
                state: CarState,
                hitbox_widths: vec3,
                hitbox_offset: vec3,
                team: i32,
                id: i32,
                controls: Input,
                I: mat3,
                invI: mat3,
            }

            impl CxxVector<Car> {}
        }

        use crate::linalg::math::{eye, inv};
        use sim_car_extra::{mat3, vec3, Input};
        pub use sim_car_extra::{Car, CarBody, CarState};

        impl Copy for CarBody {}
        impl Copy for CarState {}

        pub const M: f32 = 180.;
        pub const V_MAX: f32 = 2300.;
        pub const W_MAX: f32 = 5.5;

        impl Default for Car {
            #[inline]
            fn default() -> Self {
                let i = M * mat3::from([[751., 0., 0.], [0., 1334., 0.], [0., 0., 1836.]]);
                Self {
                    position: vec3::default(),
                    velocity: vec3::default(),
                    angular_velocity: vec3::default(),
                    orientation: eye(),
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
                    dodge_torque: vec3::default(),
                    frame: 0,
                    time: 0.,
                    body: CarBody::Octane,
                    state: CarState::OnGround,
                    hitbox_widths: vec3 {
                        data: [59.003_688_81, 42.099_704_74, 18.079_536_44],
                    },
                    hitbox_offset: vec3 {
                        data: [13.975_659_93, 0., 20.754_987_72],
                    },
                    team: 0,
                    id: 0,
                    controls: Input::default(),
                    I: i,
                    invI: inv(&i),
                }
            }
        }
    }

    pub mod ball {
        #[cxx::bridge]
        mod sim_ball {
            unsafe extern "C++" {
                include!("simulation/ball.h");

                type vec3 = crate::cvec3;
                type Ball;

                fn step(self: &mut Ball, dt: f32);
            }

            #[derive(Clone, Copy, Debug, Default)]
            struct Ball {
                position: vec3,
                velocity: vec3,
                angular_velocity: vec3,
                time: f32,
            }
        }

        pub use sim_ball::Ball;
    }

    pub mod boost_pad {
        autocxx::include_cpp! {
            #include "simulation/boost_pad.h"
            name!(sim_boost_pad)
            safety!(unsafe)
            generate_pod!("BoostPadState")
            generate_pod!("BoostPadType")
        }

        #[cxx::bridge]
        mod sim_boost_pad_extra {
            unsafe extern "C++" {
                include!("simulation/boost_pad.h");

                type vec3 = crate::cvec3;
                type BoostPadType = super::sim_boost_pad::BoostPadType;
                type BoostPadState = super::sim_boost_pad::BoostPadState;
                type BoostPad;
            }

            #[derive(Clone, Copy)]
            struct BoostPad {
                position: vec3,
                #[cxx_name = "type"]
                type_: BoostPadType,
                state: BoostPadState,
                timer: f32,
                actor_id: u16,
            }

            impl CxxVector<BoostPad> {}
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

        pub use sim_boost_pad_extra::{BoostPad, BoostPadState, BoostPadType};
    }

    pub mod goal {
        autocxx::include_cpp! {
            #include "simulation/goal.h"
            name!(sim_goal)
            safety!(unsafe)
            generate_pod!("GoalState")
        }

        #[cxx::bridge]
        mod sim_goal_extra {
            unsafe extern "C++" {
                include!("simulation/goal.h");

                type vec3 = crate::cvec3;
                type GoalState = super::sim_goal::GoalState;
                type Goal;
            }

            struct Goal {
                state: GoalState,
                position: vec3,
                direction: vec3,
                width: f32,
                height: f32,
                team: u8,
                actor_id: u16,
            }

            impl CxxVector<Goal> {}
        }

        pub use sim_goal_extra::{Goal, GoalState};
    }
}
