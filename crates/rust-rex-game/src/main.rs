use macroquad::prelude::*;

use rust_rex_core::{GameStatus, World};

mod highscore;
mod render;

const SCREEN_WIDTH: f32 = 900.0;
const SCREEN_HEIGHT: f32 = 300.0;
const GROUND_Y: f32 = 220.0;

/// Embedded (not loaded from disk at runtime) so the game runs the same way
/// no matter what directory `cargo run` / the built binary is launched from.
const PLAYER_SPRITE_BYTES: &[u8] = include_bytes!("../assets/player.png");
const OBSTACLE_SPRITE_BYTES: &[u8] = include_bytes!("../assets/obstacle.png");

/// Frames slower than this are clamped so a stall (e.g. window drag) can't
/// make the player tunnel through an obstacle in one huge simulation step.
const MAX_FRAME_TIME: f32 = 1.0 / 20.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust-Rex".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn jump_requested() -> bool {
    is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up)
}

fn restart_requested() -> bool {
    is_key_pressed(KeyCode::R) || is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space)
}

fn new_seed() -> u64 {
    (get_time() * 1_000_000.0) as u64
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut best_km = highscore::load();
    let mut world = World::new(new_seed(), SCREEN_WIDTH, GROUND_Y);
    let player_texture = Texture2D::from_file_with_format(PLAYER_SPRITE_BYTES, None);
    let obstacle_texture = Texture2D::from_file_with_format(OBSTACLE_SPRITE_BYTES, None);

    loop {
        let dt = get_frame_time().min(MAX_FRAME_TIME);

        match world.status {
            GameStatus::Running => {
                if jump_requested() {
                    world.jump();
                }
                world.update(dt);
                if world.status == GameStatus::GameOver && world.distance_km() > best_km {
                    best_km = world.distance_km();
                    highscore::save(best_km);
                }
            }
            GameStatus::GameOver => {
                if restart_requested() {
                    world.restart(new_seed());
                }
            }
        }

        render::draw(
            &world,
            &player_texture,
            &obstacle_texture,
            best_km,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            GROUND_Y,
        );

        next_frame().await;
    }
}
