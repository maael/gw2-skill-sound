# Guild Wars 2 Skill Sound Plugin

This is a Guild Wars 2 plugin that is a bit of a meme, and made by request for Trent#0655 on Discord.

## What does it do?

When core Necromancer or Reaper enter their respective Shrouds, either by key-press or being detected by ArcDPS, it will play `shroud.ogg`. It will also stop it when the shroud is exited.

> **Info**
> When detected by ArcDPS, the music playing can be significantly delayed.

> **Info**
> In an attempt to avoid this, it also reacts to keypress, although this is fixed to F1. (If F1 is bound to something else, you may have issues)

## How to install it

### Prerequisites

- ArcDPS

### Installation

1. Download and unzip the `.zip` in releases
2. Place the `shroud.ogg` in your Guild Wars 2 folder, alongside `GW2.exe`
3. Place the extension `.dll` alongside arcdps

## Todo

- Figure out how to trim down the plugin to minimal required code
  - Remove mumble link code probably? (though this currently is currently used to decide class/specialization, we may be able to get it from ArcDPS)
- Make the ArcDPS responding better (and ideally take out keypress jank)
- Make configurable - set what skills and multiple sounds to play
