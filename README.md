# global_game_jam_2022

#### Paul Easter, Avi Vajpeyi

This is our entry for the global game jam 2022

## Dependencies
* rust
* bevy


### How to use this repo:
* install rust + bevy
* requires [`wasm-server-runner`]: `cargo install wasm-server-runner`
* requires `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
* run app on you local machine (`cargo run`)
* run app in a browser (`cargo run --target wasm32-unknown-unknown`)


## CI workflow:
* workflow for GitHub actions creating releases for Windows, Linux, macOS, and Web (Wasm) ready for distribution
* push a tag in the form of `v[0-9]+.[0-9]+.[0-9]+*` (e.g. `v1.1.42`) to trigger the flow

(adapted from git@github.com:NiklasEi/bevy_game_template.git)
