use core::time::Duration;

use pebble_rust_2026::{Button, GRect, TextLayer, Timer, Window, color::GCOLOR_WHITE, fmt, heap};

use crate::{
    bitmaps::bitmaps, draw_commands::draw_commands, flash::flash, move_box::move_box,
    nested_window::nested_window, scroll::scroll,
};

pub fn heap() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);

    let mut used_layer = TextLayer::new(GRect::new(0, 0, 200, 30)).unwrap();
    window.add_child(&mut used_layer);

    let mut free_layer = TextLayer::new(GRect::new(0, 30, 200, 30)).unwrap();
    window.add_child(&mut free_layer);

    let mut total_layer = TextLayer::new(GRect::new(0, 60, 200, 30)).unwrap();
    window.add_child(&mut total_layer);

    let mut controls_layer = TextLayer::new(GRect::new(0, 90, 200, 30)).unwrap();
    window.add_child(&mut controls_layer);
    controls_layer.set_text_c_str(c"press select to allocate");

    let mut update = move || {
        let used = heap::bytes_used();
        let free = heap::bytes_free();
        let total = used + free;
        used_layer.set_text(&unsafe { fmt!(c"Used:  %lu", used) }.unwrap());
        free_layer.set_text(&unsafe { fmt!(c"Free:  %lu", free) }.unwrap());
        total_layer.set_text(&unsafe { fmt!(c"Total: %lu", total) }.unwrap());

        true
    };

    update();

    let timer = Timer::repeat(Duration::from_millis(300), update).unwrap();

    window.set_click_provider(|b| {
        b.single(
            Button::Select,
            |_| {
                drop(scroll());
                drop(nested_window(0));
                drop(draw_commands());
                drop(flash());
                drop(bitmaps());
                drop(move_box());
            },
            None,
        );
    });

    window.set_unload_handler(|| {
        timer.cancel();
    });

    window
}
