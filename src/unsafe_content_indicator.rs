use core::ptr::null_mut;
use pebble_rust_2026::{log_fmt, sys};

static mut S_SCROLL_LAYER: *mut sys::ScrollLayer = null_mut();
static mut S_INDICATOR_UP: *mut sys::Layer = null_mut();
static mut S_INDICATOR_DOWN: *mut sys::Layer = null_mut();
static mut S_CONTENT_LAYER: *mut sys::TextLayer = null_mut();

unsafe extern "C" fn window_load(window: *mut sys::Window) {
    unsafe {
        log_fmt!(
            c"size_of::<sys::ContentIndicatorConfig>(): %zu",
            size_of::<sys::ContentIndicatorConfig>()
        );
        log_fmt!(c"size_of::<sys::GColor>(): %zu", size_of::<sys::GColor>());
        log_fmt!(c"size_of::<sys::GAlign>(): %zu", size_of::<sys::GAlign>());
        let window_layer = sys::window_get_root_layer(window);
        let bounds = sys::layer_get_bounds(window_layer);

        S_SCROLL_LAYER = sys::scroll_layer_create(bounds);
        sys::scroll_layer_set_click_config_onto_window(S_SCROLL_LAYER, window);
        sys::scroll_layer_set_shadow_hidden(S_SCROLL_LAYER, true);
        sys::layer_add_child(window_layer, sys::scroll_layer_get_layer(S_SCROLL_LAYER));

        let indicator = sys::scroll_layer_get_content_indicator(S_SCROLL_LAYER);

        S_INDICATOR_UP = sys::layer_create(sys::GRect {
            origin: sys::GPoint {
                x: bounds.origin.x,
                y: bounds.origin.y,
            },
            size: sys::GSize {
                w: bounds.size.w,
                h: 16,
            },
        });
        S_INDICATOR_DOWN = sys::layer_create(sys::GRect {
            origin: sys::GPoint {
                x: 0,
                y: bounds.size.h - 16,
            },
            size: sys::GSize {
                w: bounds.size.w,
                h: 16,
            },
        });
        sys::layer_add_child(window_layer, S_INDICATOR_UP);
        sys::layer_add_child(window_layer, S_INDICATOR_DOWN);

        let up_config = sys::ContentIndicatorConfig {
            layer: S_INDICATOR_UP,
            times_out: false,
            alignment: sys::GAlign_GAlignCenter,
            colors: sys::ContentIndicatorConfig__bindgen_ty_1 {
                foreground: sys::GColor { argb: 0b11000000 }, // black
                background: sys::GColor { argb: 0b11111111 }, // white
            },
        };
        sys::content_indicator_configure_direction(
            indicator,
            sys::ContentIndicatorDirection_ContentIndicatorDirectionUp,
            &up_config,
        );

        let down_config = sys::ContentIndicatorConfig {
            layer: S_INDICATOR_DOWN,
            times_out: false,
            alignment: sys::GAlign_GAlignCenter,
            colors: sys::ContentIndicatorConfig__bindgen_ty_1 {
                foreground: sys::GColor { argb: 0b11000000 },
                background: sys::GColor { argb: 0b11111111 },
            },
        };
        sys::content_indicator_configure_direction(
            indicator,
            sys::ContentIndicatorDirection_ContentIndicatorDirectionDown,
            &down_config,
        );

        S_CONTENT_LAYER = sys::text_layer_create(sys::GRect {
            origin: bounds.origin,
            size: sys::GSize {
                w: bounds.size.w,
                h: 2000,
            },
        });
        sys::text_layer_set_text(
            S_CONTENT_LAYER,
            c"Cupcake\n\nDonut\n\nEclair\n\nFroyo\n\nGingerbread\n\nHoneycomb\n\nCupcake\n\nDonut\n\nEclair\n\nFroyo\n\nGingerbread\n\nHoneycomb".as_ptr(),
        );
        sys::text_layer_set_text_alignment(
            S_CONTENT_LAYER,
            sys::GTextAlignment_GTextAlignmentCenter,
        );
        sys::text_layer_set_font(
            S_CONTENT_LAYER,
            sys::fonts_get_system_font(sys::FONT_KEY_GOTHIC_18_BOLD.as_ptr()),
        );
        sys::scroll_layer_add_child(S_SCROLL_LAYER, sys::text_layer_get_layer(S_CONTENT_LAYER));

        let text_size = sys::text_layer_get_content_size(S_CONTENT_LAYER);
        sys::layer_set_frame(
            sys::text_layer_get_layer(S_CONTENT_LAYER),
            sys::GRect {
                origin: bounds.origin,
                size: sys::GSize {
                    w: bounds.size.w,
                    h: text_size.h,
                },
            },
        );
        sys::scroll_layer_set_content_size(S_SCROLL_LAYER, text_size);
    }
}

unsafe extern "C" fn window_unload(_window: *mut sys::Window) {
    unsafe {
        sys::scroll_layer_destroy(S_SCROLL_LAYER);
        sys::text_layer_destroy(S_CONTENT_LAYER);
        sys::layer_destroy(S_INDICATOR_UP);
        sys::layer_destroy(S_INDICATOR_DOWN);
    }
}

pub fn unsafe_content_indicator() {
    unsafe {
        let window = sys::window_create();
        let handlers = sys::WindowHandlers {
            load: Some(window_load),
            unload: Some(window_unload),
            appear: None,
            disappear: None,
        };
        sys::window_set_window_handlers(window, handlers);
        sys::window_stack_push(window, true);
    }
}
