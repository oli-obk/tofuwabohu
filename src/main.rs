use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, PI},
};

use macroquad::prelude::*;
use save::Saveable;

mod datastructures;
mod save;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tofuwabohu".to_owned(),
        window_resizable: true,
        ..Default::default()
    }
}

struct Game {
    chickens: Saveable<u64>,
    nests: Saveable<u64>,
    dogs: Saveable<u64>,
    eggs: Saveable<u64>,
    breeding: Saveable<u64>,
    corn: Saveable<u64>,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = Game {
        chickens: Saveable::new(1_u64, "chickens"),
        dogs: Saveable::new(0_u64, "dogs"),
        nests: Saveable::new(0_u64, "nests"),
        breeding: Saveable::new(0_u64, "breeding"),
        eggs: Saveable::new(0_u64, "eggs"),
        corn: Saveable::new(0_u64, "corn"),
    };
    save::transaction_loop(|| {
        let xb = screen_width() * 0.1;
        let yb = screen_height() * 0.1;

        // Logic

        if state.breeding > 1000 {
            let n = *state.breeding / 1000;
            state.breeding -= 1000 * n;
            state.chickens += n * 10;
            state.nests -= n;
        }
        state.breeding += *state.nests;

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            if (xb..(screen_width() - xb)).contains(&x) {
                if ((screen_height() - yb * 2.0)..screen_height()).contains(&y) {
                    state.eggs += *state.chickens - *state.nests;
                }
                if ((screen_height() - yb * 4.0)..(screen_height() - yb * 2.0)).contains(&y) {
                    if state.eggs > 10 {
                        if state.nests < *state.chickens {
                            state.eggs -= 10;
                            state.nests += 1;
                        }
                    }
                }
            }
        }

        // Drawing
        clear_background(BLACK);
        let thickness = (xb + yb) / 10.0;

        draw_text(
            &format!("Chickens: {}", *state.chickens),
            xb,
            yb * 2.0,
            yb,
            DARKGRAY,
        );

        if state.eggs > 0 {
            draw_text(
                &format!("Eggs: {}", *state.eggs),
                xb,
                yb * 3.0,
                yb,
                DARKGRAY,
            );
        }

        if state.nests > 0 {
            draw_text(
                &format!("Nests: {}, Breeding: {}%", *state.nests, *state.breeding * 100 / 1000),
                xb,
                yb * 4.0,
                yb,
                DARKGRAY,
            );
        }

        if state.eggs > 10 {
            draw_rectangle_lines(xb, screen_height() - yb * 4.0, screen_width() - xb * 2.0, yb * 2.0, thickness, RED);
            let dims = measure_text("Build Nest", None, yb as _, 1.0);
            draw_text(
                "Build Nest",
                screen_width() / 2.0 - dims.width / 2.0,
                screen_height() - yb * 3.0 + dims.height / 2.0,
                yb,
                RED,
            );
        }

        draw_rectangle_lines(xb, screen_height() - yb * 2.0, screen_width() - xb * 2.0, yb * 2.0, thickness, GREEN);
        let dims = measure_text("Lay Egg", None, yb as _, 1.0);
        draw_text(
            "Lay Egg",
            screen_width() / 2.0 - dims.width / 2.0,
            screen_height() - yb + dims.height / 2.0,
            yb,
            if state.chickens > *state.nests {
                GREEN
            } else {
                DARKGRAY
            },
        );

        // Let the engine actually do stuff

        next_frame()
    })
    .await;
}
