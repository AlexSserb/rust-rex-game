use rand::Rng;

use crate::geometry::Rect;

pub const OBSTACLE_MIN_SIZE: f32 = 18.0;
/// Kept low enough that even two max-size boxes stacked stay well under the
/// jump's peak height (see `player::GRAVITY` / `JUMP_VELOCITY`), leaving a
/// real timing margin instead of requiring a frame-perfect jump.
pub const OBSTACLE_MAX_SIZE: f32 = 30.0;

/// Tallest an obstacle can ever be: two max-size boxes stacked.
pub const OBSTACLE_MAX_HEIGHT: f32 = OBSTACLE_MAX_SIZE * 2.0;

/// Chance that a spawned obstacle gets a second box stacked on top.
const STACK_CHANCE: f64 = 0.35;

/// A crate-like obstacle sitting on the ground: either a single square box,
/// or two square boxes (independently sized) stacked on top of each other.
/// Each box is always a square, though the two boxes in a stack can differ
/// in size.
#[derive(Debug, Clone, Copy)]
pub struct Obstacle {
    /// Left edge of the obstacle's overall bounding box. Also the leading
    /// edge since obstacles travel right-to-left (x decreases over time).
    pub x: f32,
    /// Side length of the bottom (ground-level) box.
    pub bottom_size: f32,
    /// Side length of a second box stacked on top, if any.
    pub top_size: Option<f32>,
}

impl Obstacle {
    pub fn random(rng: &mut impl Rng, x: f32) -> Self {
        let bottom_size = rng.gen_range(OBSTACLE_MIN_SIZE..=OBSTACLE_MAX_SIZE);
        let top_size = rng
            .gen_bool(STACK_CHANCE)
            .then(|| rng.gen_range(OBSTACLE_MIN_SIZE..=OBSTACLE_MAX_SIZE));
        Self {
            x,
            bottom_size,
            top_size,
        }
    }

    /// Overall bounding width: the wider of the (up to) two stacked boxes.
    pub fn width(&self) -> f32 {
        self.bottom_size.max(self.top_size.unwrap_or(0.0))
    }

    /// Overall bounding height: both boxes' sizes summed.
    pub fn height(&self) -> f32 {
        self.bottom_size + self.top_size.unwrap_or(0.0)
    }

    pub fn bounds(&self, ground_y: f32) -> Rect {
        Rect::new(self.x, ground_y - self.height(), self.width(), self.height())
    }

    /// Bounds of each individual square box, bottom-up, each centered
    /// horizontally within the obstacle's overall bounding box. Used for
    /// drawing — every returned `Rect` has equal width and height.
    pub fn box_bounds(&self, ground_y: f32) -> Vec<Rect> {
        let width = self.width();
        let mut boxes = Vec::with_capacity(2);

        boxes.push(Rect::new(
            self.x + (width - self.bottom_size) / 2.0,
            ground_y - self.bottom_size,
            self.bottom_size,
            self.bottom_size,
        ));

        if let Some(top_size) = self.top_size {
            boxes.push(Rect::new(
                self.x + (width - top_size) / 2.0,
                ground_y - self.bottom_size - top_size,
                top_size,
                top_size,
            ));
        }

        boxes
    }

    /// True once the obstacle has fully scrolled off the left edge of the screen.
    pub fn is_off_screen(&self) -> bool {
        self.x + self.width() < 0.0
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
        let mut saw_single = false;
        let mut saw_stacked = false;
        for _ in 0..1000 {
            let obstacle = Obstacle::random(&mut rng, 800.0);
            assert!((OBSTACLE_MIN_SIZE..=OBSTACLE_MAX_SIZE).contains(&obstacle.bottom_size));
            match obstacle.top_size {
                Some(top_size) => {
                    saw_stacked = true;
                    assert!((OBSTACLE_MIN_SIZE..=OBSTACLE_MAX_SIZE).contains(&top_size));
                }
                None => saw_single = true,
            }
        }
        assert!(saw_single, "expected some single-box obstacles");
        assert!(saw_stacked, "expected some stacked obstacles");
    }

    #[test]
    fn every_box_is_a_square() {
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..1000 {
            let obstacle = Obstacle::random(&mut rng, 800.0);
            for rect in obstacle.box_bounds(250.0) {
                assert_eq!(rect.w, rect.h);
            }
        }
    }

    #[test]
    fn stacked_boxes_sit_on_top_of_each_other() {
        let obstacle = Obstacle {
            x: 100.0,
            bottom_size: 30.0,
            top_size: Some(18.0),
        };
        let boxes = obstacle.box_bounds(200.0);
        assert_eq!(boxes.len(), 2);
        let bottom = boxes[0];
        let top = boxes[1];
        assert_eq!(bottom.y + bottom.h, 200.0);
        assert_eq!(top.y + top.h, bottom.y);
        assert_eq!(obstacle.height(), bottom.h + top.h);
    }

    #[test]
    fn off_screen_detection() {
        let obstacle = Obstacle {
            x: -10.0,
            bottom_size: 9.0,
            top_size: None,
        };
        assert!(obstacle.is_off_screen());

        let obstacle = Obstacle {
            x: -10.0,
            bottom_size: 11.0,
            top_size: None,
        };
        assert!(!obstacle.is_off_screen());
    }
}
