# Mass Gathering

...is 3D gravity-based game where you, a mass in free-fall, influence the momentum of planets by firing at them with momentum-altering projectiles. The goal is to cause the planets to collide with each other. When planets collide, their masses and momentum (vectors) are averaged.

Mass Gathering is built in Rust using the Bevy game framework.

Here's a short video clip of three clients playing over the network, along with copious debug output.

[![IMAGE ALT TEXT HERE](https://img.youtube.com/vi/f4SgXuvTqWI/0.jpg)](https://www.youtube.com/watch?v=f4SgXuvTqWI)

## Running

### Inputs

#### Keyboard

`WASD`
: Pitch and Yaw

`ZX`
: Roll

`Space`
: Fire!

#### Mouse

Mouse movement for pitch and yaw. Use your keyboard (ZX keys) for roll. Click (or space) to fire.

### Running Locally

No special toolchain or manual setup is required. Just...

1. [Install Rust](https://www.rust-lang.org/tools/install).
1. Clone this repository somewhere locally.
1. `cd` to the root directory of the repository and execute `cargo run`

### WASM Builds

A WASM build playable in your browser is available [here](https://unintuitive.org/mass_gathering).

Note that you will see a blank white screen for a few seconds as the (19MB) WASM binary is transferred.

Some branches and past revisions do support WASM, "main" (not the default branch) is a good place to start.
