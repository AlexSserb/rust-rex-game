# Rust-Rex

A small T-Rex-runner-style game (the offline dinosaur game from Chrome's
"no internet" page), written in Rust. Jump crates, don't get hit — the
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
  `rust-rex-core`'s `World::update` once per frame. Sprites live in
  `crates/rust-rex-game/assets/` (`player.png`, `obstacle.png`) and are
  embedded into the binary at compile time (`include_bytes!`), so the game
  renders them correctly no matter what directory it's launched from. Each
  obstacle is one or two square crates — `obstacle.png` drawn once or twice,
  stacked — with sizes chosen independently per box.

## Play

```sh
cargo run -p rust-rex-game
```

- `Space` / `Up` — jump over a crate
- `R` / `Enter` / `Space` — restart after game over

Your best distance is saved to `rust_rex_highscore.txt` next to wherever
you run the binary from.

## Test

```sh
cargo test -p rust-rex-core
```
