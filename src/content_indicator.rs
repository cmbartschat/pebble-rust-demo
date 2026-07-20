use pebble_rust_2026::{
    ContentIndicator, ContentIndicatorConfig, ContentIndicatorDirection, GRect, Layer, Window,
    color::GCOLOR_GREEN,
};

pub fn content_indicator() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_GREEN);

    let mut indicator = ContentIndicator::new().unwrap();

    let mut down_layer = Layer::new(GRect::new(0, 60, 60, 60)).unwrap();

    let mut up_layer = Layer::new(GRect::new(0, window.get_bounds().size.h - 60, 60, 60)).unwrap();

    indicator
        .configure_direction(
            ContentIndicatorDirection::Down,
            ContentIndicatorConfig::basic(down_layer.clone()),
        )
        .unwrap();

    indicator
        .configure_direction(
            ContentIndicatorDirection::Up,
            ContentIndicatorConfig::basic(up_layer.clone()),
        )
        .unwrap();

    indicator.set_content_available(ContentIndicatorDirection::Up, true);
    indicator.set_content_available(ContentIndicatorDirection::Down, true);

    window.add_child(&mut up_layer);
    window.add_child(&mut down_layer);

    window.set_unload_handler(move || {
        drop(indicator);
    });

    window
}
