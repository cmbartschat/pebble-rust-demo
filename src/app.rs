use alloc::boxed::Box;
use pebble_rust_2026::{
    APP, SimpleMenuItem, SimpleMenuLayer, SimpleMenuSection, Window, log_c_str,
};

use crate::{bitmaps::bitmaps, draw_commands::draw_commands, flash::flash, move_box::move_box};

pub fn run_app() {
    let mut menu_window = Window::new().unwrap();

    let mut options = SimpleMenuSection::new_untitled();
    options.push(SimpleMenuItem::new("Flash", None, None, move || {
        log_c_str(c"selected option 1");
        APP.show(flash());
    }));

    options.push(SimpleMenuItem::new("Move Box", None, None, move || {
        log_c_str(c"selected option 2");
        APP.show(move_box());
    }));

    options.push(SimpleMenuItem::new(
        "Draw Commands",
        None,
        None,
        move || {
            APP.show(draw_commands());
        },
    ));

    options.push(SimpleMenuItem::new("Bitmaps", None, None, move || {
        APP.show(bitmaps());
    }));

    let mut menu = SimpleMenuLayer::new(
        menu_window.get_bounds(),
        &mut menu_window,
        Box::new([options]),
    )
    .unwrap();
    menu_window.add_child(&mut menu);

    APP.show(menu_window);

    APP.event_loop();

    log_c_str(c"finished loop");
}
