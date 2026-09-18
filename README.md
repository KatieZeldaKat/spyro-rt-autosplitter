# Spyro: Reignited Auto-Splitter

A cross-platform auto-splitter for Spyro: Reignited Trilogy.

> [!warning]
> 
> This auto-splitter has not yet been verified for leaderboard submission.

## Installation

Download the [latest release](https://github.com/KatieZeldaKat/spyro-rt-autosplitter/releases/latest) of `spyro_rt_autosplitter.wasm`.

It is highly recommended that you use [LiveSplit One Druid](https://github.com/AlexKnauth/livesplit-one-druid) ([latest release](https://github.com/AlexKnauth/livesplit-one-druid/releases/latest)).

Once you have LiveSplit One Druid open, right click on the window and select "Open Auto-splitter..." and select `spyro_rt_autosplitter.wasm`.

## Settings

With LiveSplit One Druid open, right click on the window and select "Edit Auto-splitter Settings...". This auto-splitter has a variety of settings to edit:

- Reset on Title Screen
    - On
    - Off (default)
- Split on Dragon/Egg Rescued
    - Never (default)
    - Category\*
    - Always
- Split on Boss Defeated
    - Never
    - First Defeat (default)
    - Always
- Split on Level Exit
    - Never
    - First Exit (default)
    - Always

### \*Category

The "Category" option for collectibles corresponds to those on the [Category Extensions Leaderboard](https://www.speedrun.com/spyrortce):

- Spyro the Dragon → 80 Dragons
- Spyro 2: Ripto's Rage → 64 Orbs (not yet supported)
- Spyro: Year of the Dragon → 149 Eggs

## Changes from ASL

This auto-splitter is based on the [Auto Splitting Language (ASL) version](https://github.com/SirBorris/SpyroAutoSplit). To make the auto-splitter cross-platform, it needed to be entirely re-written using the Auto Splitting Runtime (ASR). In the process of this re-write, a few changes have been made.

### Settings

ASR allows for more setting customization than simple checkboxes. Thus, it has much fewer settings than the ASL version. Some settings have also been removed from the ASR version:

- Ignore Fast Exits
    - This setting is removed in the ASR version due to a framework limitation. I suggest trying out the `FirstExit` and `Never` settings as a workaround, if possible.
- Split on Ripto Enter
    - This setting is removed in the ASR version, since including it would increase code complexity for niche behavior.

### Title to Game

When going from the title screen to one of the three Spyro games, the game time is paused until the player gains control of Spyro. This behavior is present in the ASL version, but only for the first time that particular game is selected. ASR makes this happen *every* time the player goes from the title screen to a game.

This behavior could potentially be reverted should it be deemed as too much of a deviation from the original.

## Compilation

This auto-splitter is written in Rust. In order to compile it, you need to [install the Rust compiler](https://www.rust-lang.org/tools/install).

Afterwards, install the WebAssembly target:

```bash
rustup target add wasm32-unknown-unknown
```

The auto-splitter can now be compiled:

```bash
cargo b --release --target wasm32-unknown-unknown
```

The auto-splitter is then available at:

```
target/wasm32-unknown-unknown/release/spyro_rt_autosplitter.wasm
```

## Development

You can use the [ASR Debugger](https://github.com/LiveSplit/asr-debugger) while developing the auto-splitter to more easily see the log messages, statistics, dump memory, step through the code and more.

Make sure you're using a debug version of the auto-splitter rather than a release version:

```bash
cargo b --target wasm32-unknown-unknown
```

This debug version is then available at:

```
target/wasm32-unknown-unknown/debug/spyro_rt_autosplitter.wasm
```

See also: [The `asr` crate's API documentation](https://livesplit.org/asr/asr/).

## Special Thanks

I'd like to thank the people who helped me along the way. Without them, I would never have been able to develop this version of the auto-splitter.

- [CryZe](https://github.com/CryZe) - for getting me past my initial n00b-iness in Rust
- [Bored_Banana](http://www.youtube.com/@bored_banana1) - for providing me with working memory addresses to read from
- [Alex Knauth](https://github.com/AlexKnauth) - for creating the LiveSplit Druid fork and generally helping with my understanding of LiveSplit's many quirks
