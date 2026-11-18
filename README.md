<h1 align="center">Capez for Horizon</h1>

<p align="center">
  Bring your cape along.<br>
  OptiFine capes for Horizon, inspired by the Java Capes mod.
</p>

<p align="center">
  <img alt="Minecraft 26.3" src="https://img.shields.io/badge/Minecraft-26.3-62a043?style=flat-square">
  <img alt="Native Rust mod" src="https://img.shields.io/badge/Rust-native-ce884b?style=flat-square">
  <img alt="Client side" src="https://img.shields.io/badge/side-client-5986bf?style=flat-square">
  <a href="LICENSE"><img alt="License: WTFPL" src="https://img.shields.io/badge/license-WTFPL-8b72b6?style=flat-square"></a>
</p>

<p align="center">
  <a href="#installation">Installation</a> ·
  <a href="#building">Building</a> ·
  <a href="#verification-status">Status</a> ·
  <a href="#credits--license">Credits</a>
</p>

---

See players' OptiFine capes in [Horizon](https://github.com/horizon-rust/horizon),
using the game's existing cape rendering and animations.

- **OptiFine capes** — look up cape images by player name.
- **Minecraft capes first** — keep a player's Minecraft cape when one is available.
- **Horizon integration** — use the existing cape visibility setting and texture cache.
- **Client-side** — see capes without changing the server.

## Installation

> [!IMPORTANT]
> Native mods require the **exact Horizon and mixins builds** pinned in
> [`horizon-mod.toml`](horizon-mod.toml). Matching the Minecraft version alone is insufficient.

From the mod directory, build directly into a compatible instance:

```sh
horizon-launcher native-build . --instance "YOUR INSTANCE NAME" --trust-native-code
```

Adjust the launcher path and instance name for your installation. This command
builds the native code and selects the result in the chosen instance. Native mod
code runs with your user permissions; only build code you trust.

To check your own cape, join a world or server and switch to third-person view.
Make sure **Cape** is enabled in your skin customization settings.

## Building

Run these commands from the directory containing `Cargo.toml`, with the Horizon
mod CLI installed:

```sh
# Check formatting and run the tests.
cargo fmt --check
cargo test --offline
cargo clippy --offline --all-targets -- -D warnings

# Package the mod and validate the archive.
cargo horizon-mod build
cargo horizon-mod check target/horizon-mod/capez.hmod
```

The package is written to `target/horizon-mod/capez.hmod`. The `--offline`
commands assume dependencies are cached. Package validation does not compile
or execute native mixins.

## Verification status

Cape rendering has been observed in-game. The latest texture-alignment fix has
passed automated tests, mixin application, and patched-function compilation;
it still needs an in-game visual check.

<details>
<summary><strong>Manual checks</strong></summary>

- Check your own OptiFine cape in third person and another player's cape.
- Check a player with a Minecraft cape and a player without a cape.
- Turn the cape visibility setting off and on.
- Check that the cape image is aligned correctly.

</details>

## How it works

Capez requests cape images from OptiFine by player name over HTTP, following the
Java Capes mod's approach. Horizon downloads and caches the images locally;
Capez pads them to match the Java mod's texture layout.

Other cape providers, the Java mod's settings screens, and its cape editor are
outside Capez's scope.

## Credits & license

Inspired by [Capes](https://github.com/CaelTheColher/Capes) by Cael and
contributors, distributed under the GNU LGPL 2.1. Capez uses the Java mod as a
reference for OptiFine cape lookup and image layout.

Capez is licensed under the **WTFPL** — see [LICENSE](LICENSE).

### Disclaimer
This mod has been entierly written by Codex.
