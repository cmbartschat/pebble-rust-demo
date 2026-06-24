use pebble_rust_2026::{
    GContext, GPoint, GRect, Layer, Window,
    color::{
        GCOLOR_DARK_GREEN, GCOLOR_GREEN, GCOLOR_SUNSET_ORANGE, GCOLOR_VERY_LIGHT_BLUE,
        GCOLOR_WHITE, GCOLOR_YELLOW,
    },
    sys,
};

extern "C" fn draw_to_layer(_layer: *mut sys::Layer, ctx: *mut sys::GContext) {
    let mut ctx = GContext::from_raw(ctx).unwrap();

    ctx.set_fill_color(GCOLOR_VERY_LIGHT_BLUE);
    ctx.fill_rect(GRect::new(0, 0, 200, 100));

    ctx.set_fill_color(GCOLOR_YELLOW);
    ctx.fill_circle(GPoint { x: 50, y: 50 }, 25);

    ctx.set_fill_color(GCOLOR_GREEN);
    ctx.fill_rect(GRect::new(0, 100, 200, 150));

    ctx.set_fill_color(GCOLOR_SUNSET_ORANGE);
    ctx.fill_round_rect(GRect::new(140, 100, 20, 100), 3);

    ctx.set_fill_color(GCOLOR_DARK_GREEN);
    ctx.fill_round_rect(GRect::new(120, 50, 60, 100), 10);
}

pub fn draw_commands() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);

    {
        let mut custom_layer = Layer::new(window.get_bounds()).unwrap();
        custom_layer.set_update_proc(draw_to_layer);
        window.add_child(&mut custom_layer);
    }

    window
}
