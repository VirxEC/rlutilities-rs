pub mod rlu {
    autocxx::include_cpp! {
        #include "rlutilities.h"
        name!(base)
        safety!(unsafe_ffi)
        generate!("rlu::initialize")
    }

    pub use base::rlu::*;
}

pub mod linear_algebra {
    pub mod math {
        autocxx::include_cpp! {
            #include "linear_algebra/math.h"
            name!(lin_math)
            safety!(unsafe_ffi)
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
            safety!(unsafe_ffi)
            generate!("Game")
        }

        pub use sim_game::*;
    }

    pub mod ball {
        autocxx::include_cpp! {
            #include "simulation/ball.h"
            name!(sim_ball)
            safety!(unsafe_ffi)
            generate!("BallShape")
            generate!("Ball")
        }

        pub use sim_ball::*;
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
