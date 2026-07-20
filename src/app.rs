use alloc::boxed::Box;
use pebble_rust_2026::{
    APP, SimpleMenuItem, SimpleMenuLayer, SimpleMenuSection, Window, log_c_str,
};

use crate::{
    bitmaps::bitmaps, content_indicator::content_indicator, draw_commands::draw_commands,
    flash::flash, heap::heap, move_box::move_box, scroll::scroll, spin::spin, time::time,
    unsafe_content_indicator::unsafe_content_indicator,
};

pub fn run_app() {
    let mut menu_window = Window::new().unwrap();

    let mut options = SimpleMenuSection::new_untitled();
    options.push(SimpleMenuItem::new("Content", None, None, move || {
        APP.show(content_indicator());
    }));
    options.push(SimpleMenuItem::new("Scroll", None, None, move || {
        APP.show(scroll());
    }));
    options.push(SimpleMenuItem::new("Heap", None, None, move || {
        APP.show(heap());
    }));
    options.push(SimpleMenuItem::new("Time", None, None, move || {
        APP.show(time());
    }));
    options.push(SimpleMenuItem::new("Spin", None, None, move || {
        APP.show(spin());
    }));

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
    // APP.show(content_indicator());
    // unsafe_content_indicator();

    APP.event_loop();

    log_c_str(c"finished loop");
}
