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
    eggs: Saveable<u64>,
    breeding: Saveable<u64>,
}

#[derive(Default)]
struct Messages {
    msgs: Vec<String>,
}

#[derive(Default)]
struct Buttons {
    buttons: Vec<Button>,
}

struct Button {
    label: String,
    action: Option<Box<dyn FnOnce(&mut Game)>>,
    color: Color,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = Game {
        chickens: Saveable::new(1_u64, "chickens"),
        nests: Saveable::new(0_u64, "nests"),
        breeding: Saveable::new(0_u64, "breeding"),
        eggs: Saveable::new(0_u64, "eggs"),
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

        // Drawing
        clear_background(BLACK);
        let thickness = (xb + yb) / 10.0;

        let mut messages = Messages::default();
        messages.msgs.push(format!("Chickens: {}", *state.chickens));

        if state.eggs > 0 {
            messages.msgs.push(format!("Eggs: {}", *state.eggs));
        }

        if state.nests > 0 {
            messages.msgs.push(format!("Nests: {}", *state.nests));
            messages
                .msgs
                .push(format!("Breeding: {}%", *state.breeding * 100 / 1000));
        }

        for (i, msg) in messages.msgs.iter().enumerate() {
            draw_text(msg, xb, yb * (i + 1) as f32 * 0.9, yb, DARKGRAY);
        }

        let mut buttons = Buttons::default();

        if state.eggs > 10 {
            buttons.buttons.push(Button {
                label: "Build Nest".into(),
                action: Some(Box::new(|state| {
                    if state.nests < *state.chickens {
                        state.eggs -= 10;
                        state.nests += 1;
                    }
                })),
                color: RED,
            });
        }

        buttons.buttons.push(Button {
            label: "Lay Egg".into(),
            action: Some(Box::new(|state| {
                state.eggs += *state.chickens - *state.nests;
            })),
            color: GREEN,
        });

        let button_height = yb * 1.5;
        let button_width = screen_width() - xb * 2.0;
        for (i, button) in buttons.buttons.into_iter().rev().enumerate() {
            draw_rectangle_lines(
                xb,
                screen_height() - button_height * (i + 1) as f32,
                button_width,
                button_height,
                thickness,
                button.color,
            );
            let dims = measure_text(&button.label, None, button_height as _, 1.0);
            let scale_x = button_width / dims.width;
            let scale_y = button_height / dims.height * 0.9;
            let scale = if scale_x < scale_y { scale_x } else { scale_y } / 1.5;
            draw_text(
                &button.label,
                screen_width() / 2.0 - dims.width * scale / 2.0,
                screen_height() - button_height * i as f32 - dims.height * scale / 2.0,
                button_height * scale,
                if button.action.is_some() {
                    button.color
                } else {
                    DARKGRAY
                },
            );

            if let Some(action) = button.action {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (x, y) = mouse_position();
                    if (xb..(screen_width() - xb)).contains(&x) {
                        if ((screen_height() - button_height * (i + 1) as f32)
                            ..(screen_height() - button_height * i as f32))
                            .contains(&y)
                        {
                            action(&mut state)
                        }
                    }
                }
            }
        }

        // Let the engine actually do stuff

        next_frame()
    })
    .await;
}
