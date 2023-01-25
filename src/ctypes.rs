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
        #[allow(non_camel_case_types)]
        pub struct vec3 {
            pub data: [f32; 3],
        }

        unsafe impl cxx::ExternType for vec3 {
            type Id = cxx::type_id!("vec3");
            type Kind = cxx::kind::Trivial;
        }

        #[cxx::bridge]
        mod linalg_vec {
            unsafe extern "C++" {
                include!("linear_algebra/vec.h");

                #[allow(dead_code)]
                type vec3 = super::vec3;
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
            extern_cpp_opaque_type!("Ball", crate::sim::ball::Ball)
            generate!("Game")
        }

        #[cxx::bridge]
        mod sim_game_extra {
            unsafe extern "C++" {
                include!("simulation/game.h");

                type vec3 = crate::linalg::vec::vec3;
                type Game = super::sim_game::Game;

                #[cxx_name = "reset_pad"]
                fn reset_pad_2(
                    self: Pin<&mut Game>,
                    index: i32,
                    position: vec3,
                    is_full_boost: bool,
                );

                #[cxx_name = "reset_goal"]
                fn reset_goal_2(
                    self: Pin<&mut Game>,
                    index: i32,
                    position: vec3,
                    direction: vec3,
                    width: f32,
                    height: f32,
                    team: i32,
                );
            }
        }

        autocxx::include_cpp! {
            #include "simulation/car.h"
            name!(sim_car)
            safety!(unsafe)
            block!("vec3")
            generate!("Car")
        }

        pub use sim_game_extra::*;
    }

    pub mod ball {
        autocxx::include_cpp! {
            #include "simulation/ball.h"
            name!(sim_ball)
            safety!(unsafe)
            // generate_pod!("BallShape")
            generate!("Ball")
        }

        #[cxx::bridge]
        mod sim_ball_extra {
            unsafe extern "C++" {
                include!("simulation/ball.h");

                type vec3 = crate::linalg::vec::vec3;
                type Ball = super::sim_ball::Ball;

                #[cxx_name = "get_position"]
                fn get_position_2(self: &Ball) -> vec3;

                #[cxx_name = "set_position"]
                fn set_position_2(self: Pin<&mut Ball>, pos: vec3);

                #[cxx_name = "get_velocity"]
                fn get_velocity_2(self: &Ball) -> vec3;

                #[cxx_name = "set_velocity"]
                fn set_velocity_2(self: Pin<&mut Ball>, vel: vec3);

                #[cxx_name = "get_angular_velocity"]
                fn get_angular_velocity_2(self: &Ball) -> vec3;

                #[cxx_name = "set_angular_velocity"]
                fn set_angular_velocity_2(self: Pin<&mut Ball>, ang_vel: vec3);
            }
        }

        pub use sim_ball_extra::*;
    }

    pub mod boost_pad {
        autocxx::include_cpp! {
            #include "simulation/boost_pad.h"
            name!(sim_boost_pad)
            safety!(unsafe_ffi)
            generate!("BoostPad")
            generate_pod!("BoostPadState")
            generate_pod!("BoostPadType")
        }

        pub use sim_boost_pad::*;
    }
}
