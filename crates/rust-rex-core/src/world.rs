use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::obstacle::Obstacle;
use crate::player::Player;

/// Scroll speed at the start of a run, in px/s.
pub const BASE_SPEED: f32 = 300.0;

/// Scroll speed never ramps past this, in px/s, so the game stays
/// playable no matter how long a run lasts.
pub const MAX_SPEED: f32 = 520.0;

/// How much the scroll speed increases per meter traveled, in (px/s) per
/// meter. Speed ramps linearly from `BASE_SPEED` and reaches `MAX_SPEED`
/// after `(MAX_SPEED - BASE_SPEED) / SPEED_RAMP_PER_METER` meters — about
/// 440m (roughly half a minute of play), so difficulty creeps up instead
/// of maxing out within the first few seconds.
const SPEED_RAMP_PER_METER: f32 = 0.5;

/// How many horizontal pixels of travel correspond to one meter of
/// in-game distance. Purely a display/scale choice.
pub const PIXELS_PER_METER: f32 = 25.0;

/// Obstacle spawn gaps are chosen in *time*, not pixels, and converted to
/// pixels using the speed at spawn time. That keeps the player's reaction
/// window roughly constant even as the scroll speed ramps up — a fixed
/// pixel gap would shrink in time (and become unfair) as the game sped up.
const MIN_SPAWN_GAP_SECONDS: f32 = 1.2;
const MAX_SPAWN_GAP_SECONDS: f32 = 2.2;

/// Fixed x position (in world/screen space) of the player. Only obstacles move.
pub const PLAYER_X: f32 = 60.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Running,
    GameOver,
}

/// Ties player physics, obstacle spawning/movement and collision detection
/// together into one updatable game state. Rendering-agnostic: a frontend
/// only needs to call [`World::update`] each frame and read the public
/// fields to draw the scene.
pub struct World {
    pub player: Player,
    pub obstacles: Vec<Obstacle>,
    pub status: GameStatus,
    pub distance_traveled_px: f32,
    pub ground_y: f32,
    screen_width: f32,
    rng: StdRng,
    next_spawn_at_px: f32,
}

impl World {
    pub fn new(seed: u64, screen_width: f32, ground_y: f32) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let next_spawn_at_px =
            BASE_SPEED * rng.gen_range(MIN_SPAWN_GAP_SECONDS..=MAX_SPAWN_GAP_SECONDS);
        Self {
            player: Player::new(),
            obstacles: Vec::new(),
            status: GameStatus::Running,
            distance_traveled_px: 0.0,
            ground_y,
            screen_width,
            rng,
            next_spawn_at_px,
        }
    }

    pub fn jump(&mut self) {
        if self.status == GameStatus::Running {
            self.player.jump();
        }
    }

    /// Resets the world in place, reusing the same screen geometry but
    /// reseeding the obstacle pattern.
    pub fn restart(&mut self, seed: u64) {
        *self = World::new(seed, self.screen_width, self.ground_y);
    }

    pub fn update(&mut self, dt: f32) {
        if self.status != GameStatus::Running {
            return;
        }

        let speed = self.current_speed();
        self.player.update(dt);
        self.distance_traveled_px += speed * dt;

        for obstacle in &mut self.obstacles {
            obstacle.x -= speed * dt;
        }
        self.obstacles.retain(|o| !o.is_off_screen());

        if self.distance_traveled_px >= self.next_spawn_at_px {
            self.obstacles
                .push(Obstacle::random(&mut self.rng, self.screen_width));
            let gap_seconds = self
                .rng
                .gen_range(MIN_SPAWN_GAP_SECONDS..=MAX_SPAWN_GAP_SECONDS);
            self.next_spawn_at_px = self.distance_traveled_px + speed * gap_seconds;
        }

        let player_bounds = self.player.bounds(PLAYER_X, self.ground_y);
        let collided = self
            .obstacles
            .iter()
            .any(|o| player_bounds.intersects(&o.bounds(self.ground_y)));
        if collided {
            self.status = GameStatus::GameOver;
        }
    }

    pub fn distance_meters(&self) -> f32 {
        self.distance_traveled_px / PIXELS_PER_METER
    }

    pub fn distance_km(&self) -> f32 {
        self.distance_meters() / 1000.0
    }

    /// Current scroll speed, in px/s. Ramps linearly with distance
    /// traveled so runs start manageable and gradually get harder,
    /// capped at `MAX_SPEED`.
    pub fn current_speed(&self) -> f32 {
        (BASE_SPEED + SPEED_RAMP_PER_METER * self.distance_meters()).min(MAX_SPEED)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_accumulates_using_the_current_speed() {
        let mut world = World::new(1, 800.0, 250.0);
        let speed = world.current_speed();
        world.update(1.0);
        assert_eq!(world.distance_traveled_px, speed);
    }

    #[test]
    fn starts_at_base_speed() {
        let world = World::new(2, 800.0, 250.0);
        assert_eq!(world.current_speed(), BASE_SPEED);
    }

    #[test]
    fn speed_ramps_up_with_distance_and_caps_at_max_speed() {
        let mut world = World::new(3, 800.0, 250.0);

        world.distance_traveled_px = PIXELS_PER_METER * 10.0;
        let speed_after_10m = world.current_speed();
        assert!(speed_after_10m > BASE_SPEED);
        assert!(speed_after_10m < MAX_SPEED);

        world.distance_traveled_px = PIXELS_PER_METER * 100_000.0;
        assert_eq!(world.current_speed(), MAX_SPEED);
    }

    #[test]
    fn speed_ramp_is_monotonic_over_a_run() {
        let mut world = World::new(4, 800.0, 250.0);
        let mut last_speed = world.current_speed();
        for _ in 0..600 {
            world.update(1.0 / 60.0);
            if world.status == GameStatus::GameOver {
                world.restart(4);
                last_speed = world.current_speed();
                continue;
            }
            let speed = world.current_speed();
            assert!(speed >= last_speed);
            last_speed = speed;
        }
    }

    #[test]
    fn obstacles_are_eventually_spawned_and_cleaned_up() {
        let mut world = World::new(3, 800.0, 250.0);
        for _ in 0..600 {
            world.update(1.0 / 30.0);
        }
        assert!(!world.obstacles.is_empty() || world.status == GameStatus::GameOver);
        for obstacle in &world.obstacles {
            assert!(!obstacle.is_off_screen());
        }
    }

    #[test]
    fn collision_ends_the_game() {
        let mut world = World::new(4, 800.0, 250.0);
        world.obstacles.push(Obstacle {
            x: PLAYER_X,
            bottom_size: 50.0,
            top_size: None,
        });
        world.update(0.0);
        assert_eq!(world.status, GameStatus::GameOver);
    }

    #[test]
    fn jumping_over_a_low_obstacle_avoids_collision() {
        let mut world = World::new(5, 800.0, 250.0);
        world.jump();
        // Let the jump gain some height before the obstacle arrives under the player.
        for _ in 0..10 {
            world.update(1.0 / 60.0);
        }
        world.obstacles.push(Obstacle {
            x: PLAYER_X,
            bottom_size: 30.0,
            top_size: None,
        });
        world.update(0.0);
        assert_eq!(world.status, GameStatus::Running);
    }

    #[test]
    fn update_is_a_noop_after_game_over() {
        let mut world = World::new(6, 800.0, 250.0);
        world.status = GameStatus::GameOver;
        let distance_before = world.distance_traveled_px;
        world.update(1.0);
        assert_eq!(world.distance_traveled_px, distance_before);
    }

    #[test]
    fn distance_km_matches_pixel_scale() {
        let mut world = World::new(7, 800.0, 250.0);
        world.distance_traveled_px = PIXELS_PER_METER * 1000.0;
        assert!((world.distance_km() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn restart_resets_progress() {
        let mut world = World::new(8, 800.0, 250.0);
        world.update(2.0);
        assert!(world.distance_traveled_px > 0.0);
        world.restart(8);
        assert_eq!(world.distance_traveled_px, 0.0);
        assert_eq!(world.status, GameStatus::Running);
        assert!(world.obstacles.is_empty());
    }
}
