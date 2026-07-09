# same timezone ✈️

Thirteen days, three cities, one question: when are you free?

A schedule-sync app for a September trip to Mexico — she sees the route, taps
the days she's free, optionally picks legs to tag along on, and copies a
summary back. Written in Rust with [Leptos](https://leptos.dev), compiled to
WASM, bundled by [Trunk](https://trunkrs.dev).

## Architecture in one breath

Leptos uses **fine-grained reactivity**: every `RwSignal` knows exactly which
DOM nodes read it, so tapping a day updates only that cell. No virtual DOM, no
diffing.

Deliberate from-scratch choices, inherited from the last build:

- **No `chrono`.** Weekdays come from Zeller's congruence (`src/util.rs`),
  unit-tested against known dates — including the one that matters.
- **No `rand`.** Confetti positions come from a hand-rolled xorshift64* PRNG
  seeded from the browser clock.
- **Illegal states are unrepresentable.** The flow is an `enum Step`; you
  cannot pick vibes before seeing the route. The compiler is the chaperone.

## Run it

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk

trunk serve            # dev server at http://localhost:8081, rebuilds on save
cargo test             # date math + range compression tests
trunk build --release  # optimized WASM into ./dist
```

## Deploy

`trunk build --release` produces a static `dist/` — ship it to any static
host that serves `.wasm` as `application/wasm`:

```sh
npx wrangler pages deploy dist --project-name same-timezone
```
