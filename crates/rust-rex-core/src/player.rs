use crate::geometry::Rect;

pub const PLAYER_WIDTH: f32 = 44.0;
pub const PLAYER_HEIGHT: f32 = 47.0;

/// Downward acceleration applied while airborne, in px/s^2. Tuned high so
/// the jump is a short, snappy hop (~0.55s total airtime, ~100px peak
/// height) rather than a long floaty arc — the previous, gentler gravity
/// made every jump feel sluggish and hard to time against obstacles.
const GRAVITY: f32 = -2600.0;
/// Upward velocity applied the instant a jump starts, in px/s.
const JUMP_VELOCITY: f32 = 720.0;

/// The runner. Horizontal position is fixed on screen; only height above
/// the ground (`y`) changes, driven by simple projectile motion.
#[derive(Debug, Clone, Copy)]
pub struct Player {
    /// Height above the ground line. `0.0` means standing on the ground.
    pub y: f32,
    velocity_y: f32,
    pub is_jumping: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            y: 0.0,
            velocity_y: 0.0,
            is_jumping: false,
        }
    }

    /// Starts a jump. Ignored if the player is already airborne, so holding
    /// the jump key does not chain into a double jump.
    pub fn jump(&mut self) {
        if !self.is_jumping {
            self.velocity_y = JUMP_VELOCITY;
            self.is_jumping = true;
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_jumping {
            return;
        }
        self.velocity_y += GRAVITY * dt;
        self.y += self.velocity_y * dt;
        if self.y <= 0.0 {
            self.y = 0.0;
            self.velocity_y = 0.0;
            self.is_jumping = false;
        }
    }

    /// Collision bounds, given the fixed on-screen x position of the player
    /// and the world-space y coordinate of the ground line.
    pub fn bounds(&self, x: f32, ground_y: f32) -> Rect {
        Rect::new(
            x,
            ground_y - self.y - PLAYER_HEIGHT,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_leaves_the_ground_then_returns() {
        let mut player = Player::new();
        player.jump();
        assert!(player.is_jumping);

        for _ in 0..200 {
            player.update(1.0 / 60.0);
        }

        assert!(!player.is_jumping);
        assert_eq!(player.y, 0.0);
    }

    #[test]
    fn jump_is_ignored_while_airborne() {
        let mut player = Player::new();
        player.jump();
        player.update(1.0 / 60.0);
        let velocity_after_first_jump = player.velocity_y;

        player.jump();
        assert_eq!(player.velocity_y, velocity_after_first_jump);
    }

    #[test]
    fn idle_player_does_not_move() {
        let mut player = Player::new();
        player.update(1.0 / 60.0);
        assert_eq!(player.y, 0.0);
        assert!(!player.is_jumping);
    }

    #[test]
    fn jump_clears_the_tallest_obstacle_with_margin() {
        let mut player = Player::new();
        player.jump();
        let mut peak = 0.0f32;
        while player.is_jumping {
            player.update(1.0 / 240.0);
            peak = peak.max(player.y);
        }
        assert!(peak > crate::obstacle::OBSTACLE_MAX_HEIGHT + 20.0);
    }

    #[test]
    fn jump_is_short_and_snappy() {
        let mut player = Player::new();
        player.jump();
        let mut airtime = 0.0f32;
        let dt = 1.0 / 240.0;
        while player.is_jumping {
            player.update(dt);
            airtime += dt;
        }
        assert!(airtime < 0.7);
    }
}
