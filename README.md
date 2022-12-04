# See the main branch for the broader story...

## Networking Works

You should be able to start both a client and server and have them do things.

```
cargo run # server
```

```
cargo run --bin client
```

Note that the windows will open on top of each other. "In practice" we aim not to have a gui on the server (of course).

As of now, watch for:

1. Server starts, arranges its planets.
1. Server sends all the physics and PBR info to the client.
1. Client spawns the planets, based on the above network data.
1. Client notifies server "I am ready" (we would in practice wait for "everyone")
1. Server sends command to clients instructing them to start their local simulations.
1. Server starts itself _also_.
1. Client receives command to start, starts.

Visually the simulations are identical, they will vary in practice just a bit (latency, [in]precision creep). We will probably keep things simple and crude like this and just send update-data on a regular schedule (something generous. 1000ms..?)

Since the clients need to apply this info immediately (and since we'll be doing sync-ups regardless) we'll just let there be a bit of a jump on the fist update. Note also that these updates will be much smaller, like maybe `(id, position, velocity)`.

What remains:

* Make the server headless, efficient.
* Have clients inhabit certain distinct-looking planets with their cameras.
* Would-be-nice: have a small "picture-in-picture" server view making the player positions apparent.
* All the other stuff. Missile impacts, crosshairs, some simple togglable info-board.
* A greeting page when we are waiting for a game to begin/enough payers.
* blockchain technology