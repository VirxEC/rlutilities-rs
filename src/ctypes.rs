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
            generate!("Game")
        }

        pub use sim_game::*;
    }

    pub mod ball {
        autocxx::include_cpp! {
            #include "simulation/ball.h"
            name!(sim_ball)
            safety!(unsafe)
            // generate_pod!("BallShape")
            block!("Ball::get_position")
            generate!("Ball")
        }

        #[cxx::bridge]
        mod sim_ball_2 {
            unsafe extern "C++" {
                include!("simulation/ball.h");

                type Ball;

                #[cxx_name = "get_position"]
                fn get_position_2(&self) -> [f32; 3];
            }
        }

        pub use sim_ball::*;
        // pub use sim_ball_2::*;
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
