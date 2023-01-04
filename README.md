# Mass Gathering [WIP]

A 3D gravity-based game where you (a planet) try to bring about collisions with other planets with momentum-altering missiles.

Here is a pretty looking video of three clients and one server (top-right window, camera at the origin). The clients are each represented by one of the "planets" you can see and they can gimbal and roll with keyboard input, hence the view of the universe reflects this. Log output to the far left...

[![IMAGE ALT TEXT HERE](https://img.youtube.com/vi/f4SgXuvTqWI/0.jpg)](https://www.youtube.com/watch?v=f4SgXuvTqWI)

_A WASM-capable, 3D, n-body simulation game written in [Rust](https://www.rust-lang.org/) using [Bevy](https://bevyengine.org/)._

## Gameplay

Your spacecraft spawns into a newly-formed solar system with chaotic orbits. Your job is to claim as much _mass_ as you can.

You do this by:

* Shooting momentum-altering projectiles at planets. If you strike a planet, you claim it.
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
