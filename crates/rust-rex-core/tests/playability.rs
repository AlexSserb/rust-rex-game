//! A property-style regression guard: a simple reactive bot (jump when an
//! obstacle gets close enough, given current scroll speed) should be able
//! to survive a long run across many random seeds. If tuning changes make
//! the game unfair — speed ramping too fast, obstacle gaps too tight,
//! jump arc too short to clear anything — this is designed to catch it
//! without needing to play the game by hand.

use rust_rex_core::{GameStatus, World, PLAYER_X};

const DT: f32 = 1.0 / 60.0;
const TARGET_SURVIVAL_SECONDS: f32 = 60.0;
const SEEDS: std::ops::Range<u64> = 0..200;

/// How far ahead (in seconds of travel at the current speed) the bot
/// reacts to an incoming obstacle and jumps.
const REACTION_LEAD_SECONDS: f32 = 0.32;

#[test]
fn a_reactive_bot_survives_a_full_minute_across_many_seeds() {
    for seed in SEEDS {
        let mut world = World::new(seed, 900.0, 220.0);
        let mut survived = 0.0f32;

        while survived < TARGET_SURVIVAL_SECONDS {
            let reaction_distance = world.current_speed() * REACTION_LEAD_SECONDS;
            let obstacle_incoming = world.obstacles.iter().any(|o| {
                let ahead = o.x - PLAYER_X;
                (0.0..=reaction_distance).contains(&ahead)
            });
            if obstacle_incoming {
                world.jump();
            }

            world.update(DT);
            survived += DT;

            assert_ne!(
                world.status,
                GameStatus::GameOver,
                "seed {seed}: bot died after {survived:.2}s (speed {:.0}px/s, distance {:.1}m)",
                world.current_speed(),
                world.distance_meters(),
            );
        }
    }
}
