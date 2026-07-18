use core::time::Duration;

use pebble_rust_2026::{GRect, TextLayer, Timer, Window, color::GCOLOR_WHITE, fmt, heap};

pub fn heap() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);

    let mut used_layer = TextLayer::new(GRect::new(0, 0, 200, 30)).unwrap();
    window.add_child(&mut used_layer);

    let mut free_layer = TextLayer::new(GRect::new(0, 30, 200, 30)).unwrap();
    window.add_child(&mut free_layer);

    let mut total_layer = TextLayer::new(GRect::new(0, 60, 200, 30)).unwrap();
    window.add_child(&mut total_layer);

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

    let timer = Timer::repeat(Duration::from_secs(3), update).unwrap();

    window.set_unload_handler(|| {
        timer.cancel();
    });

    window
}
