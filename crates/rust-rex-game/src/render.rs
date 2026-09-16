//! Everything macroquad-specific: turning a [`World`] snapshot into pixels.
//! Kept separate from `main` so the game loop stays readable, and from
//! `rust-rex-core` so the simulation never depends on a rendering backend.

use macroquad::prelude::*;

use rust_rex_core::{GameStatus, World, PLAYER_X};

const BACKGROUND: Color = Color::new(0.97, 0.97, 0.95, 1.0);
const GROUND_COLOR: Color = Color::new(0.2, 0.2, 0.2, 1.0);
const TEXT_COLOR: Color = Color::new(0.15, 0.15, 0.15, 1.0);

const GROUND_TICK_SPACING: f32 = 40.0;
const GROUND_TICK_WIDTH: f32 = 18.0;

/// The player sprite is drawn taller than its collision box (which stays a
/// tight rectangle for fair, predictable collisions) so the gear-dino
/// artwork isn't squashed to fit it. `PLAYER_SPRITE_SCALE` controls how much
/// bigger, anchored to the same ground contact point and horizontal center
/// as the hitbox.
const PLAYER_SPRITE_SCALE: f32 = 1.35;

pub fn draw(
    world: &World,
    player_texture: &Texture2D,
    obstacle_texture: &Texture2D,
    best_km: f32,
    screen_width: f32,
    screen_height: f32,
    ground_y: f32,
) {
    clear_background(BACKGROUND);

    draw_ground(world, screen_width, ground_y);
    draw_player(world, player_texture, ground_y);
    draw_obstacles(world, obstacle_texture, ground_y);
    draw_hud(world, best_km, screen_width);

    if world.status == GameStatus::GameOver {
        draw_game_over(screen_width, screen_height);
    }
}

fn draw_ground(world: &World, screen_width: f32, ground_y: f32) {
    draw_line(0.0, ground_y, screen_width, ground_y, 2.0, GROUND_COLOR);

    // Short scrolling ticks below the line so the constant speed still
    // reads as motion even though the ground itself has no obstacles.
    let offset = world.distance_traveled_px % GROUND_TICK_SPACING;
    let mut x = -offset;
    while x < screen_width {
        draw_line(
            x,
            ground_y + 6.0,
            x + GROUND_TICK_WIDTH,
            ground_y + 6.0,
            2.0,
            GROUND_COLOR,
        );
        x += GROUND_TICK_SPACING;
    }
}

fn draw_player(world: &World, player_texture: &Texture2D, ground_y: f32) {
    let body = world.player.bounds(PLAYER_X, ground_y);

    let draw_h = body.h * PLAYER_SPRITE_SCALE;
    let draw_w = draw_h * (player_texture.width() / player_texture.height());
    let center_x = body.x + body.w / 2.0;
    let ground_contact = body.y + body.h;

    draw_texture_ex(
        player_texture,
        center_x - draw_w / 2.0,
        ground_contact - draw_h,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(draw_w, draw_h)),
            ..Default::default()
        },
    );
}

fn draw_obstacles(world: &World, obstacle_texture: &Texture2D, ground_y: f32) {
    for obstacle in &world.obstacles {
        // Each obstacle is one or two square crates stacked on the ground;
        // every box is drawn as its own copy of the crate texture, sized to
        // that box's own (square) bounds.
        for rect in obstacle.box_bounds(ground_y) {
            draw_texture_ex(
                obstacle_texture,
                rect.x,
                rect.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(rect.w, rect.h)),
                    ..Default::default()
                },
            );
        }
    }
}

fn draw_hud(world: &World, best_km: f32, screen_width: f32) {
    let current = format!("{:.2} km", world.distance_km());
    let best = format!("BEST {:.2} km", best_km);

    let current_dims = measure_text(&current, None, 24, 1.0);
    draw_text(
        &current,
        screen_width - current_dims.width - 16.0,
        28.0,
        24.0,
        TEXT_COLOR,
    );

    let best_dims = measure_text(&best, None, 18, 1.0);
    draw_text(
        &best,
        screen_width - best_dims.width - 16.0,
        50.0,
        18.0,
        TEXT_COLOR,
    );
}

fn draw_game_over(screen_width: f32, screen_height: f32) {
    let title = "GAME OVER";
    let subtitle = "Press R or Enter to restart";

    let title_dims = measure_text(title, None, 36, 1.0);
    draw_text(
        title,
        (screen_width - title_dims.width) / 2.0,
        screen_height / 2.0 - 10.0,
        36.0,
        TEXT_COLOR,
    );

    let subtitle_dims = measure_text(subtitle, None, 20, 1.0);
    draw_text(
        subtitle,
        (screen_width - subtitle_dims.width) / 2.0,
        screen_height / 2.0 + 24.0,
        20.0,
        TEXT_COLOR,
    );
}
