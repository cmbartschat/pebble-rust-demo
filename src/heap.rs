use core::time::Duration;

use alloc::{boxed::Box, vec};
use pebble_rust_2026::{
    APP, Button, GRect, TextLayer, Timer, Window, color::GCOLOR_WHITE, fmt, heap,
};

use crate::windows::WINDOWS;

pub fn heap() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);

    let mut used_layer = TextLayer::new(GRect::new(0, 0, 200, 30)).unwrap();
    window.add_child(&mut used_layer);

    let mut free_layer = TextLayer::new(GRect::new(0, 30, 200, 30)).unwrap();
    window.add_child(&mut free_layer);

    let mut total_layer = TextLayer::new(GRect::new(0, 60, 200, 30)).unwrap();
    window.add_child(&mut total_layer);

    let mut instruction_layer = TextLayer::new(GRect::new(0, 90, 200, 100)).unwrap();
    instruction_layer.set_text_c_str(c"Press Up to init\nPress Select to mount");
    window.add_child(&mut instruction_layer);

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

    window.set_click_provider(|b| {
        b.single(
            Button::Up,
            |_| {
                for (_, f) in WINDOWS {
                    f();
                }
            },
            None,
        );
        b.single(
            Button::Select,
            |_| {
                let mut windows = vec![];
                for (_, f) in WINDOWS {
                    windows.push(f());
                }

                windows.reverse();

                for w in windows.iter_mut() {
                    APP.show_immediate(w.retain());
                }

                Timer::once(Duration::from_millis(1000), move || {
                    for mut w in windows.into_iter() {
                        APP.hide_immediate(&mut w);
                    }
                });
            },
            None,
        );
    });

    window.set_appear_effect(Box::new(move || {
        Box::new({
            let timer = Timer::repeat(Duration::from_millis(300), update.clone()).unwrap();
            Box::new(move || {
                timer.cancel();
            })
        })
    }));

    window
}
