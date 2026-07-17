use alloc::string::String;
use pebble_rust_2026::{APP, GRect, TextLayer, Time, TimeUnits, Window, color::GCOLOR_WHITE, fmt};

pub fn time() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);

    let mut local_time_layer = TextLayer::new(GRect::new(0, 0, 200, 30)).unwrap();
    window.add_child(&mut local_time_layer);

    let mut utc_time_layer = TextLayer::new(GRect::new(0, 30, 200, 30)).unwrap();
    window.add_child(&mut utc_time_layer);

    let mut now_timestamp_layer = TextLayer::new(GRect::new(0, 60, 200, 30)).unwrap();
    window.add_child(&mut now_timestamp_layer);

    let mut converted_local_timestamp_layer = TextLayer::new(GRect::new(0, 90, 200, 30)).unwrap();
    window.add_child(&mut converted_local_timestamp_layer);

    let mut converted_utc_timestamp_layer = TextLayer::new(GRect::new(0, 120, 200, 30)).unwrap();
    window.add_child(&mut converted_utc_timestamp_layer);

    APP.set_tick_handler(TimeUnits::Second, move || {
        let now = Time::now();
        local_time_layer.set_text_bytes(now.to_local().format_hh_mm().as_bytes());
        utc_time_layer.set_text_bytes(now.to_utc().format_hh_mm().as_bytes());
        now_timestamp_layer.set_text(&unsafe { fmt!(c"%ld", now.epoch_seconds()).unwrap() });
        converted_local_timestamp_layer.set_text(&unsafe {
            fmt!(
                c"%ld",
                Time::try_from(now.to_local()).unwrap().epoch_seconds()
            )
            .unwrap_or_else(|| String::from("<fail>"))
        });
        converted_utc_timestamp_layer.set_text(&unsafe {
            fmt!(
                c"%ld",
                Time::try_from(now.to_utc()).unwrap().epoch_seconds()
            )
            .unwrap()
        });
    });

    window
}
