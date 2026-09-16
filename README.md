# Rust-Rex

A small T-Rex-runner-style game (the offline dinosaur game from Chrome's
"no internet" page), written in Rust. Jump cacti, don't get hit — the
scroll speed ramps up gradually with distance (and caps out so it stays
playable), and the score is the distance traveled, shown in kilometers.

## Layout

This is a Cargo workspace with two crates:

- `crates/rust-rex-core` — pure game logic: player physics, obstacle
  spawning and movement, collision detection, difficulty ramping and
  distance tracking. No rendering or windowing dependencies, so it's
  fully unit-tested in isolation.
- `crates/rust-rex-game` — the playable binary. Uses [`macroquad`](https://docs.rs/macroquad)
  for the window, drawing and keyboard input, and drives
  `rust-rex-core`'s `World::update` once per frame.

## Play

```sh
cargo run -p rust-rex-game
```

- `Space` / `Up` — jump over a cactus
- `R` / `Enter` / `Space` — restart after game over

Your best distance is saved to `rust_rex_highscore.txt` next to wherever
you run the binary from.

## Test

```sh
cargo test -p rust-rex-core
```
