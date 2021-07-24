use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, PI},
};

use macroquad::prelude::*;

mod datastructures;
mod save;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tofuwabohu".to_owned(),
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    save::transaction_loop(|| {
        // Logic

        let mut cam = Camera2D::default();
        cam.zoom.x = 1.0 / (screen_width() / 2.0);
        cam.zoom.y = -1.0 / (screen_height() / 2.0);
        set_camera(&cam);

        // Drawing
        clear_background(BLACK);


        // Let the engine actually do stuff

        next_frame()
    })
    .await;
}
