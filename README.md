# Mass Gathering [WIP]

A 3D gravity-based game where you (a planet) try to bring about collisions with other planets with momentum-altering missiles.

Here is a pretty looking video of three clients and one server (top-right window, camera at the origin). The clients are each represented by one of the "planets" you can see and they can gimbal and roll with keyboard input, hence the view of the universe reflects this. Log output to the far left...

[![IMAGE ALT TEXT HERE](https://img.youtube.com/vi/f4SgXuvTqWI/0.jpg)](https://www.youtube.com/watch?v=f4SgXuvTqWI)

## Emergency Glossary

Mostly in the spirit of making the code easier to understand...

* *mass* -- To keep things general, the balls or spheres or planets you see in the game are in the code referred to as "masses", you can substitute "planet" in your head if you like. The reasoning is: you could use endless things in place of a _sphere_. You just need a center-of-mass. It could just as easily be a red roadster.
* *inhabitant* -- When a "solar system" is created on a client, a certain number of planets are "inhabitable" (has the `Inhabitable` component), that is, the camera of a client that is assigned to this mass is positioned its center (In fact
it is a "child" of this mass's `PointMassBundle`.) The spheres that you _see_ in this game are surrounding a 
_point_ mass, with a global X, Y, and Z. There is just one `ClientInhabited` entity on any client. That is 
_yours_ and _your_ camera is pinned to that.
* *hot* -- The very center of your screen points at the target for your missiles. You point your gun and your camera together, so when a _mass_ passes in front of your gun sights it "arms" your gun (you may shoot). When any target is in sight (up to a certain number of units ahead of you).

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
