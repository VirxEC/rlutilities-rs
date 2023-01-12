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
        autocxx::include_cpp! {
            #include "linear_algebra/vec.h"
            name!(lin_math)
            safety!(unsafe)
            generate!("vec3")
        }

        pub use lin_math::*;
    }
}

pub mod simulation {
    pub mod game {
        autocxx::include_cpp! {
            #include "simulation/game.h"
            name!(sim_game)
            safety!(unsafe)
            extern_cpp_opaque_type!("Ball", crate::ctypes::simulation::ball::Ball)
            generate!("Game")
        }

        #[cxx::bridge]
        mod sim_game_extra {
            unsafe extern "C++" {
                include!("simulation/game.h");

                type Game = super::sim_game::Game;

                #[cxx_name = "reset_pad"]
                fn reset_pad_2(
                    self: Pin<&mut Game>,
                    index: i32,
                    position: [f32; 3],
                    is_full_boost: bool,
                );

                #[cxx_name = "reset_goal"]
                fn reset_goal_2(
                    self: Pin<&mut Game>,
                    index: i32,
                    position: [f32; 3],
                    direction: [f32; 3],
                    width: f32,
                    height: f32,
                    team: i32,
                );
            }
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

                type Ball = super::sim_ball::Ball;

                #[cxx_name = "get_position"]
                fn get_position_2(&self) -> [f32; 3];
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
