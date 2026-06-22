use pebble_rust_2026::{APP, log_c_str};

use crate::{flash::flash, move_box::move_box};

pub fn run_app() {
    {
        APP.show(flash());
    }

    {
        APP.show(move_box());
    }

    APP.event_loop();

    log_c_str(c"finished loop");
}
