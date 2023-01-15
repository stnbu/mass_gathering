# Mass Gathering [WIP]

A 3D gravity-based game where you (a planet) try to bring about collisions with other planets with momentum-altering missiles.

Here is a pretty looking video of three clients and one server (top-right window, camera at the origin). The clients are each represented by one of the "planets" you can see and they can gimbal and roll with keyboard input, hence the view of the universe reflects this. Log output to the far left...

[![IMAGE ALT TEXT HERE](https://img.youtube.com/vi/f4SgXuvTqWI/0.jpg)](https://www.youtube.com/watch?v=f4SgXuvTqWI)

## Emergency Glossary

Mostly in the spirit of making the code easier to understand...

* **mass** -- To keep things general, the balls or spheres or planets you see in the game are in the code referred to as "masses", you can substitute "planet" in your head if you like. The reasoning is: you could use endless things in place of a _sphere_. You just need a center-of-mass. It could just as easily be a red roadster.
* **inhabitant** -- When a "solar system" is created on a client, a certain number of planets are "inhabitable" (has the `Inhabitable` component), that is, the camera of a client that is assigned to this mass is positioned its center (In fact
it is a "child" of this mass's `PointMassBundle`.) The spheres that you _see_ in this game are surrounding a 
_point_ mass, with a global X, Y, and Z. There is just one `ClientInhabited` entity on any client. That is 
_yours_ and _your_ camera is pinned to that.
* **hot** -- The very center of your screen points at the target for your missiles. You point your gun and your camera together, so when a _mass_ passes in front of your gun sights it "arms" your gun (you may shoot). When any target is in sight (up to a certain number of units ahead of you).

## Run Locally

No special toolchain or manual setup is required. Just...

1. [Install Rust](https://www.rust-lang.org/tools/install).
1. Clone this repository somewhere locally.
1. `cd` to the root directory of the repository and run `./demo.sh player1 player2`

The final command will run the server and two clients.

The server stays in the foreground of your console. The clients will be two
separate windows that open and display _your_ universe. (Note the two windows
will be on top of one another.)

If you click on a client window and bring it into focus, you can control "your planet":

`WASD`
: Pitch and Yaw

`ZX`
: Roll

`Space`
: Fire!

You should be able to see each player's "missiles" in the other window.

**IMPORTANT** -- The server slam your CPU. Be sure to `ctrl-C` in your console and kill the server when you're done. You can also `killall -9 server`.

Please file a [GitHub Issue](https://github.com/stnbu/mass_gathering/issues/new/choose) if you encounter anything "interesting".

## WASM

The plan was to target WASM. You can go back in this repo's history
and successfully compile/run many variations on this game in WASM.

You will notice they are all single player with zero networking.

Adding networking support broke this. Currently this project
uses the [] crate for game networking. This crate is simple, logical
and easy to reason with. It works great! However, it uses raw UDP
sockets which WASM does not support, at least in the browser.

My current thinking is that we can substitute WebSockets in place
of UDP. That's a very mixed bag, but the data flow should be light
and there are tricks to deal with the increased latency.

WASM would be so farging cool. It's still in the works!