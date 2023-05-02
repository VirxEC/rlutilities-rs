use once_cell::sync::Lazy;
use rlutilities_rs::*;

static INIT: Lazy<()> = Lazy::new(|| rlu::initialize("assets/"));

#[test]
fn init() {
    // ensure that the init function is called
    #[allow(clippy::no_effect)]
    *INIT;
}
