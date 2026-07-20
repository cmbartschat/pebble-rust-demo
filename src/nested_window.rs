use pebble_rust_2026::{APP, Button, GColor, TextLayer, Window, fmt, hex_color, log_fmt};

static COLORS: [GColor; 3] = [hex_color!("#ff0"), hex_color!("#f0f"), hex_color!("#0ff")];

pub fn nested_window(i: usize) -> Window {
    let mut window = Window::new().unwrap();

    let mut text_layer = TextLayer::new(window.get_bounds()).unwrap();
    text_layer.set_background_color(COLORS[i % COLORS.len()]);
    text_layer.set_text(&unsafe { fmt!(c"Level %u", i).unwrap() });
    window.add_child(&mut text_layer);

    window.set_appear_handler(move || unsafe {
        log_fmt!(c"set_appear_handler: %u", i);
    });

    window.set_disappear_handler(move || unsafe {
        log_fmt!(c"set_disappear_handler: %u", i);
    });

    window.set_load_handler(move || unsafe {
        log_fmt!(c"set_load_handler: %u", i);
    });

    window.set_unload_handler(move || unsafe {
        log_fmt!(c"set_unload_handler: %u", i);
    });

    window.set_click_provider(move |b| {
        b.single(
            Button::Select,
            move |_| {
                APP.show(nested_window(i + 1));
            },
            None,
        );
    });

    window
}
