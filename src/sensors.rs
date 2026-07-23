use core::ffi::{c_int, c_uint};

use alloc::{boxed::Box, vec::Vec};
use pebble_rust_2026::{
    APP, AccelAxis, AccelSamplingRate, GRect, TextLayer, Window,
    color::GCOLOR_WHITE,
    fmt, hex_color, log_c_str,
    sys::{self},
};

pub fn sensors() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);

    let mut offset = 0;

    let mut update_battery = {
        let mut layer = TextLayer::new(GRect::new(0, offset, 200, 30)).unwrap();
        window.add_child(&mut layer);
        offset += 30;
        move |charge_percent: u8| {
            layer.set_text(&unsafe { fmt!(c"Battery: %d%%", charge_percent as c_uint).unwrap() });
        }
    };

    let mut update_bluetooth = {
        let mut layer = TextLayer::new(GRect::new(0, offset, 200, 30)).unwrap();
        window.add_child(&mut layer);
        offset += 30;
        move |connected: bool| {
            if connected {
                layer.set_text_c_str(c"Bluetooth: Connected");
            } else {
                layer.set_text_c_str(c"Bluetooth: Not Connected");
            }
        }
    };

    let mut update_focus = {
        let mut layer = TextLayer::new(GRect::new(0, offset, 200, 30)).unwrap();
        offset += 30;
        window.add_child(&mut layer);
        move |focused: bool| {
            if focused {
                layer.set_text_c_str(c"Focused: true");
            } else {
                layer.set_text_c_str(c"Focused: false");
            }
        }
    };

    let mut update_accel = {
        let mut layer = TextLayer::new(GRect::new(0, offset, 200, 30)).unwrap();
        window.add_child(&mut layer);
        move |data: &[sys::AccelData]| {
            if let Some(data) = data.last() {
                layer.set_text(&unsafe {
                    fmt!(
                        c"Accel: (%i, %i, %i)",
                        data.x as c_int,
                        data.y as c_int,
                        data.z as c_int
                    )
                    .unwrap()
                });
            } else {
                layer.set_text_c_str(c"Accelerometer: Unavailable");
            }
        }
    };

    let weak_window = window.downgrade();
    window.set_appear_effect(Box::new(move || {
        update_battery(APP.battery_state.peek().charge_percent);
        update_bluetooth(APP.bluetooth_connection.peek());
        let data: Vec<_> = APP.accel.peek().into_iter().collect();
        update_accel(&data);

        APP.battery_state.subscribe(Box::new({
            let mut update_battery = update_battery.clone();
            move |b| update_battery(b.charge_percent)
        }));
        APP.bluetooth_connection
            .subscribe(Box::new(update_bluetooth.clone()));
        update_focus(true); // Assumed

        APP.accel.tap_subscribe(Box::new({
            let weak_window = weak_window.clone();
            move |axis| {
                let Some(mut window) = weak_window.upgrade() else {
                    return;
                };
                let color = match axis {
                    AccelAxis::PosX => hex_color!("#f0f"),
                    AccelAxis::PosY => hex_color!("#ff0"),
                    AccelAxis::PosZ => hex_color!("#0ff"),
                    AccelAxis::NegX => hex_color!("#f00"),
                    AccelAxis::NegY => hex_color!("#0f0"),
                    AccelAxis::NegZ => hex_color!("#00f"),
                };
                window.set_background_color(color);
            }
        }));
        APP.accel.subscribe(1, Box::new(update_accel.clone()));
        APP.accel.set_sampling_rate(AccelSamplingRate::Hz10);
        APP.focus.subscribe(Box::new(update_focus.clone()));

        Box::new(move || {
            APP.accel.unsubscribe();
            APP.bluetooth_connection.unsubscribe();
            APP.battery_state.unsubscribe();
            APP.focus.unsubscribe();
        })
    }));

    window
}
