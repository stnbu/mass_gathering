# Mass Gathering

...is 3D gravity-based game where you (a planet) try to bring about collisions with other planets with momentum-altering missiles.

Here's a short video clip of four clients playing over the network, along with copious debug output...

[![IMAGE ALT TEXT HERE](https://img.youtube.com/vi/f4SgXuvTqWI/0.jpg)](https://www.youtube.com/watch?v=f4SgXuvTqWI)

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

**IMPORTANT** -- The server will slam your CPU. Be sure to `ctrl-C` in your console and kill the server when you're done. You can also `killall -9 server`.

Please file a [GitHub Issue](https://github.com/stnbu/mass_gathering/issues/new/choose) if you encounter anything "interesting".

## WASM

WASM does work.

### ALL LINKS

https://unintuitive.org/mass_gathering

https://unintuitive.org/mass_gathering/tmp/relax-its-just-an-arrow

https://unintuitive.org/mass_gathering/tmp/codename_les-bogs

https://unintuitive.org/mass_gathering/tmp/cone-heads

https://unintuitive.org/mass_gathering/tmp/phlights_of_phancy

https://unintuitive.org/mass_gathering/builds/2d5f83a88f5087bb08b7bc2ae111bc56dfb28c46/examples/shooting_gallery

https://unintuitive.org/mass_gathering/builds/2d5f83a88f5087bb08b7bc2ae111bc56dfb28c46

https://unintuitive.org/mass_gathering/examples/shooting_gallery-3d

https://unintuitive.org/mass_gathering/examples/shooting_gallery

https://unintuitive.org/mass_gathering-3d

https://unintuitive.org/mass_gathering-lesigh

https://unintuitive.org/mass_gathering-_hud-momentum-vectors

https://unintuitive.org/mass_gathering-flying_wallstud

https://unintuitive.org/mass_gathering-debug
