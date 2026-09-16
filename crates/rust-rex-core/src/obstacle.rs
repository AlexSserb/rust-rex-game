use rand::Rng;

use crate::geometry::Rect;

pub const OBSTACLE_MIN_WIDTH: f32 = 18.0;
pub const OBSTACLE_MAX_WIDTH: f32 = 34.0;
pub const OBSTACLE_MIN_HEIGHT: f32 = 28.0;
/// Kept well under the jump's peak height (see `player::GRAVITY` /
/// `JUMP_VELOCITY`) so clearing even the tallest obstacle leaves a real
/// timing margin instead of requiring a frame-perfect jump.
pub const OBSTACLE_MAX_HEIGHT: f32 = 52.0;

/// A single cactus-like obstacle sitting on the ground.
#[derive(Debug, Clone, Copy)]
pub struct Obstacle {
    /// Left edge x position, which is also the leading edge since
    /// obstacles travel right-to-left (x decreases over time).
    pub x: f32,
    pub width: f32,
    pub height: f32,
}

impl Obstacle {
    pub fn random(rng: &mut impl Rng, x: f32) -> Self {
        Self {
            x,
            width: rng.gen_range(OBSTACLE_MIN_WIDTH..=OBSTACLE_MAX_WIDTH),
            height: rng.gen_range(OBSTACLE_MIN_HEIGHT..=OBSTACLE_MAX_HEIGHT),
        }
    }

    pub fn bounds(&self, ground_y: f32) -> Rect {
        Rect::new(self.x, ground_y - self.height, self.width, self.height)
    }

    /// True once the obstacle has fully scrolled off the left edge of the screen.
    pub fn is_off_screen(&self) -> bool {
        self.x + self.width < 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn random_obstacle_dimensions_stay_in_range() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..1000 {
            let obstacle = Obstacle::random(&mut rng, 800.0);
            assert!((OBSTACLE_MIN_WIDTH..=OBSTACLE_MAX_WIDTH).contains(&obstacle.width));
            assert!((OBSTACLE_MIN_HEIGHT..=OBSTACLE_MAX_HEIGHT).contains(&obstacle.height));
        }
    }

    #[test]
    fn off_screen_detection() {
        let obstacle = Obstacle {
            x: -10.0,
            width: 9.0,
            height: 40.0,
        };
        assert!(obstacle.is_off_screen());

        let obstacle = Obstacle {
            x: -10.0,
            width: 11.0,
            height: 40.0,
        };
        assert!(!obstacle.is_off_screen());
    }
}
