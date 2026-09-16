//! Rendering-agnostic game logic for Rust-Rex, a T-Rex-runner-style
//! endless game: a jumping player, scrolling obstacles whose speed ramps
//! up with distance, collision detection and distance tracking. A
//! frontend (e.g. the `rust-rex-game` binary) drives this by calling
//! [`World::update`] once per frame and reading back the public state to
//! draw the scene.

mod geometry;
mod obstacle;
mod player;
mod world;

pub use geometry::Rect;
pub use obstacle::{Obstacle, OBSTACLE_MAX_HEIGHT, OBSTACLE_MAX_SIZE, OBSTACLE_MIN_SIZE};
pub use player::{Player, PLAYER_HEIGHT, PLAYER_WIDTH};
pub use world::{GameStatus, World, BASE_SPEED, MAX_SPEED, PIXELS_PER_METER, PLAYER_X};
