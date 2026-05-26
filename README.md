# `taxipatch`

Patch for Crazy Taxi 3. Successor to CT3Tweaks.

> [!WARNING]
> No stability is guaranteed yet. Will probably run fine, just don't be surprised.

## Requirements

Only tested with the Fairlight `CT3.exe` executable. Others may work or crash at random.

> [!NOTE]
> If you have used CT3Tweaks before on your game, make sure to remove the patched version and restore the backup (rename
> it to `CT3.exe` instead). CT3Tweaks modifies the original game file, which may be incompatible with this patch.

## Installation

1. Install [Ultimate ASI Loader] (dinput8.dll version, others may work, not tested)
2. Get `taxipatch`:
    * Release version from GitHub Releases (TODO - not yet released)
    * [Development version from CI][GitHub Actions runs]
    * Build it yourself
3. Put `taxipatch.asi` inside the `plugins` folder (that is next to `CT3.exe`)
4. Start game

## Patches

#### Window

Automatically detects your primary monitor's resolution and refresh rate. Makes the game window run in that resolution.
You may configure the resolution the game runs at by setting `resolution` or `refresh_rate` properties in the config.

#### that's it for now

## Configuration

`taxipatch.ini`

```ini
[window]
resolution = "2560x1440" # uses primary monitor resolution by default
refresh_rate = 240 # same as resolution

```

[Ultimate ASI Loader]: https://github.com/ThirteenAG/Ultimate-ASI-Loader

[GitHub Actions runs]: https://github.com/stashymane/taxipatch/actions/workflows/ci.yml
