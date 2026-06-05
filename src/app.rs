use pebble_rust_2026::{APP, log_c_str};

use crate::flash::flash;

pub fn run_app() {
    {
        APP.show(flash());
    }

    APP.event_loop();

    log_c_str(c"finished loop");
}
