pub mod rlu {
    autocxx::include_cpp! {
        #include "rlutilities.h"
        name!(base)
        safety!(unsafe)
        generate!("rlu::initialize")
    }

    pub use base::rlu::*;
}

pub mod linear_algebra {
    pub mod vec {
        #[cxx::bridge]
        mod linalg_vec {
            #[derive(Clone, Debug, Default)]
            struct vec3 {
                data: [f32; 3],
            }
        }

        pub use linalg_vec::*;
    }
}

pub mod mechanics {
    pub mod drive {
        autocxx::include_cpp! {
            #include "mechanics/drive.h"
            name!(mech_drive)
            safety!(unsafe)
            generate!("Drive")
        }

        pub use mech_drive::*;
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

                type vec3 = crate::linalg::vec::vec3;
                type BoostPad = crate::sim::boost_pad::BoostPad;
                type GameState = super::sim_game::GameState;
                type Ball = crate::sim::ball::Ball;
                type Goal = crate::sim::goal::Goal;
                type Game;

                fn set_mode(gamemode: String);

                fn new_boostpad_vec() -> UniquePtr<CxxVector<BoostPad>>;
                fn new_goal_vec() -> UniquePtr<CxxVector<Goal>>;

                fn resize_goals(self: &mut Game, num_goals: i32);

                fn reset_goal(
                    self: &mut Game,
                    index: i32,
                    position: vec3,
                    direction: vec3,
                    width: f32,
                    height: f32,
                    team: i32,
                );
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
            }
        }

        pub use sim_game_extra::*;
    }

    pub mod car {
        autocxx::include_cpp! {
            #include "simulation/car.h"
            name!(sim_car)
            safety!(unsafe)
            block!("vec3")
            generate!("Car")
            generate_pod!("CarBody")
            generate_pod!("CarState")
        }

        pub use sim_car::*;
    }

    pub mod ball {
        #[cxx::bridge]
        mod sim_ball {
            unsafe extern "C++" {
                include!("simulation/ball.h");

                type vec3 = crate::linalg::vec::vec3;
                type Ball;

                fn step(self: &mut Ball, dt: f32);
            }

            #[derive(Clone, Debug, Default)]
            struct Ball {
                position: vec3,
                velocity: vec3,
                angular_velocity: vec3,
                time: f32,
            }
        }

        pub use sim_ball::*;
    }

    pub mod boost_pad {
        autocxx::include_cpp! {
            #include "simulation/boost_pad.h"
            name!(sim_boost_pad)
            safety!(unsafe)
            generate_pod!("BoostPadState")
            generate_pod!("BoostPadType")
        }

        impl From<bool> for sim_boost_pad::BoostPadType {
            fn from(value: bool) -> Self {
                if value {
                    Self::Full
                } else {
                    Self::Partial
                }
            }
        }

        #[cxx::bridge]
        mod sim_boost_pad_extra {
            unsafe extern "C++" {
                include!("simulation/boost_pad.h");

                type vec3 = crate::linalg::vec::vec3;
                type BoostPadType = super::sim_boost_pad::BoostPadType;
                type BoostPadState = super::sim_boost_pad::BoostPadState;
                type BoostPad;
            }

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

        pub use sim_boost_pad_extra::*;
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

                type vec3 = crate::linalg::vec::vec3;
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

        pub use sim_goal_extra::*;
    }
}
