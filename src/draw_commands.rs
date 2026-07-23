use core::{cell::RefCell, time::Duration};

use alloc::{boxed::Box, rc::Rc, vec::Vec};
use pebble_rust_2026::{
    APP, Bitmap, Button, CompOp, GContext, GPoint, GRect, Layer, Mutex, MutexToken, Random,
    TextLayer, TouchEvent, Window,
    color::{GCOLOR_DARK_GREEN, GCOLOR_GREEN, GCOLOR_SUNSET_ORANGE, GCOLOR_WHITE},
    hex_color, resource_ids, sys,
};

struct Bird {
    bounds: GRect,
    sprite: Bitmap,
}

impl Bird {
    pub fn new(center: GPoint, sprite: Bitmap) -> Self {
        let bounds = sprite.get_bounds();
        let bounds = GRect::new(
            center.x - bounds.size.w / 2,
            center.y - bounds.size.h / 2,
            bounds.size.w,
            bounds.size.h,
        );
        Self { bounds, sprite }
    }
    pub fn draw(&self, ctx: &mut GContext) {
        ctx.set_compositing_mode(CompOp::Set);
        ctx.draw_bitmap(&self.sprite, self.bounds);
    }
}

resource_ids!(resource_ids);

static BIRDS: Mutex<RefCell<Vec<Bird>>> = Mutex::new(RefCell::new(Vec::new()));

extern "C" fn draw_to_layer(_layer: *mut sys::Layer, ctx: *mut sys::GContext) {
    let mut ctx = GContext::from_raw(ctx).unwrap();

    ctx.set_fill_color(hex_color!("#aff"));
    ctx.fill_rect(GRect::new(0, 0, 200, 100));

    ctx.set_fill_color(hex_color!("#ffff00"));
    ctx.fill_circle(GPoint { x: 50, y: 50 }, 25);

    ctx.set_fill_color(GCOLOR_GREEN);
    ctx.fill_rect(GRect::new(0, 100, 200, 150));

    MutexToken::with(|t| {
        for fly in BIRDS.borrow(t).iter() {
            fly.draw(&mut ctx);
        }
    });

    ctx.set_fill_color(GCOLOR_SUNSET_ORANGE);
    ctx.fill_round_rect(GRect::new(140, 100, 20, 100), 3);

    ctx.set_fill_color(GCOLOR_DARK_GREEN);
    ctx.fill_round_rect(GRect::new(120, 50, 60, 100), 10);
}

pub fn draw_commands() -> Window {
    let mut window = Window::new().unwrap();
    window.set_background_color(GCOLOR_WHITE);

    let mut custom_layer = Layer::new(window.get_bounds()).unwrap();
    custom_layer.set_update_proc(draw_to_layer);
    window.add_child(&mut custom_layer);

    {
        let mut label_layer =
            TextLayer::new(GRect::new(10, window.get_bounds().size.h - 40, 180, 40)).unwrap();
        if APP.touch.is_enabled() {
            label_layer.set_text_c_str(c"draw commands\nselect or tap to place bird");
        } else {
            label_layer.set_text_c_str(c"draw commands\nselect to place bird");
        }
        label_layer.set_background_color(hex_color!("#0000"));
        window.add_child(&mut label_layer);
    }

    let weak_window = window.downgrade();
    let bird_sprites = [
        Bitmap::from_resource(resource_ids::BIRD1).unwrap(),
        Bitmap::from_resource(resource_ids::BIRD2).unwrap(),
        Bitmap::from_resource(resource_ids::BIRD3).unwrap(),
    ];

    let push_bird = Rc::new(RefCell::new(move |position: GPoint| {
        let sprite =
            bird_sprites[Random::new().uniform(bird_sprites.len() as u32) as usize].clone();

        MutexToken::with(|t| BIRDS.borrow_mut(t).push(Bird::new(position, sprite)));
        custom_layer.mark_dirty();
    }));

    let push_bird_1 = push_bird.clone();
    window.set_click_provider({
        move |b| {
            let push_bird = push_bird_1.clone();
            b.single(
                Button::Select,
                {
                    let weak_window = weak_window.clone();
                    move |_| {
                        let Some(window) = weak_window.upgrade() else {
                            return;
                        };

                        let mut bounds = window.get_bounds();
                        bounds.size.h = 120;
                        let excluded_bounds = GRect::new(120, 50, 60, 200);

                        let position = loop {
                            let position = GPoint {
                                x: Random::new().uniform(bounds.size.w as u32) as i16 - 8,
                                y: Random::new().uniform(120) as i16 - 8,
                            };

                            if !excluded_bounds.contains_point(position) {
                                break position;
                            }
                        };

                        (push_bird.borrow_mut())(position);
                    }
                },
                Some(Duration::from_millis(500)),
            );
        }
    });

    window.set_appear_effect(Box::new(move || {
        APP.touch.subscribe(Box::new({
            let push_bird = push_bird.clone();
            move |event| {
                if let TouchEvent::TouchDown(position) = event {
                    (push_bird.borrow_mut())(position);
                };
            }
        }));

        Box::new(|| {
            APP.touch.unsubscribe();
        })
    }));

    window.set_unload_handler(|| {
        MutexToken::with(|t| {
            BIRDS.borrow_mut(t).clear();
        });
    });

    window
}
