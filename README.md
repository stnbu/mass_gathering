# Mass Gathering [WIP]

_A WASM-capable, 3D, n-body simulation game written in [Rust](https://www.rust-lang.org/) using [Bevy](https://bevyengine.org/)._

## Gameplay

Your spacecraft spawns into a newly-formed solar system with chaotic orbits. Your job is to claim as much _mass_ as you can.

You do this by:

* Shooting momentum-altering projectiles at planets. If you strike a planet, you clam it.
* If a planet you have claimed collides with an unclaimed planet, the newly-formed planet becomes yours. <sup>*</sup>
* If a planet you have claimed collides with a planet claimed by another player, the owner is determined by mass: largest planet wins.
* When all planets merge into one, the game is over and, the owner of the last planet wins. <sup>**</sup>

The key bindings are displayed on-screen in the upper-left corner.

There are definitely bugs. Please do not hesitate to file a [GitHub Issue](https://github.com/stnbu/mass_gathering/issues/new/choose) or start a [Discussion](https://github.com/stnbu/mass_gathering/discussions/new).

To play, you can do one of...

### A) Play [here](https://unintuitive.org/mass_gathering/).

The WASM binary is 19MB. You will see a blank page as this loads. Once the game loads...

  1. Click anywhere in the game window (the WASM widow does not have focus until you do.)
  1. Click your space bar to un-pause (and re-pause) the game.

### B) Compile and run locally.

No special toolchain or manual setup is required. Just...

1. [Install Rust](https://www.rust-lang.org/tools/install).
1. Clone this repository somewhere locally.
1. `cd` to the root directory of the repository and execute the command `cargo run`.

Please file a [GitHub Issue](https://github.com/stnbu/mass_gathering/issues/new/choose) if you have difficulty compiling the project.

### Notes

> `*`   - Masses combine, radius grows in proportion.
>
> `**`  - Oops. According to the above gameplay, _some particular player_ will own the largest planet,
>         meaning that the winner is pre-determined long before the game ends over.


---

## Ideas/Misc

* Firmament, with a few stars at least
* Snow: render some fuzzy tiny specs only in-view, de-allocate when they disappear. Maybe give a bit of Brownian motion.
* The game:
  * ✅ They merge when they touch
  * ✅ You have infinite thrust, infinite fuel and can thrust fore and aft
  * You can _nudge_ anything with one unit. You have a 10 nudge capacity that is replenished one nudge every 3 seconds
  * If you strike a ball, you reflect
  * You get "ownership" of anything you nudge
  * Ownership is contagious:
    * If a thing you own contacts an unowned thing, you get ownership of unowned thing
    * if a thing you own contacts a thing owned by ANOTHER PLAYER, ownership is determined by mass (more massive body gets total ownership.)
