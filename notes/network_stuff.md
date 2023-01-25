# Here I attempt to explain how networking works, in various ways, to prove I have a plan.

## Sequence

When a client connects to the server, the following "states" happen, in order. All messages are currently sent over a "reliable" channel, so we take advantage of the implied reliability (Hopefully we get a message or a crash!).

It's assumed the server is configured and running and that the client is starting up (because that's when connection happens, by design):

### 00

Client calls `new_renet_client(client_id: u64, address: String) -> RenetClient`. The client is calculated by the nickname supplied by the client and visa versa as necessary (eight bytes, with right-padding in spaces, `0x20` converted to/from `u64`.) Address has a loopback default.

Connection happens automatically (asynchronously ...? there is a `run_if_client_connected` convenience system criteria.)

The returned `RenetClient` resource is inserted with `App.add_resource()`.

### 01

The server receives a `ClientConnected` event with the connected client's id and an empty "`user_data`" parameter.

`<record-screech>`

Then this shit happens:

```
                let new_client_id = *id;
                server.send_message(
                    new_client_id,
                    DefaultChannel::Reliable,
                    bincode::serialize(&events::ToClient::Init(init_data.clone())).unwrap(),
                );
                server.send_message(
                    new_client_id,
                    DefaultChannel::Reliable,
                    bincode::serialize(&events::ToClient::SetPhysicsConfig(*physics_config))
                        .unwrap(),
                );
                for (&existing_id, &client_data) in lobby.clients.iter() {
                    server.send_message(
                        new_client_id,
                        DefaultChannel::Reliable,
                        bincode::serialize(&events::ToClient::ClientJoined {
                            id: existing_id,
                            client_data,
                        })
                        .unwrap(),
                    );
                }
```

Here is the redesign:

The server makes an entry in its `Lobby` for the new client.

The server broadcasts the now-updated `Lobby` resource to everyone, this will include the client that has just connected.

### 02

The client receives the `Lobby` resource over the network and adds a clone of it as its own `Lobby`. This happens with every client that connects, which is overly verbose and "inefficient", but it stops after the last client joins after which the game begins.

After the `Lobby` resource is received and updated, the client sends a `Ready` to indicate that it is _ready_ to participate.

### 03

When the server receives the `Ready` it decrements the count of missing clients.

If the number of "missing clients" is greater than zero, i.e. the game has not yet reached the required capacity, the clients are instructed to set their state to `Waiting` with a broadcast to all clients. This may be redundant, as the first client that connected has already recieved this instruction. No problemo.

If the number of "missing clients" is zero, a single broadcast is made instructing clients to set their state to `Running`.

In either of the above case, the server sets its _own_ state after the broadcast. The intent is to guarantee we do not get more than one `Running` and that a `Waiting` cannot happen after the `Running`, the server also will use these states to start/stop its own simulation of the game.

A "waiting screen" is displayed while `Waiting`. The masses begin to move and the waiting screen disappears as soon as `Ready` is set by the client.

### 04

The game just keeps going in this "state" until something crashes or the first client disconnects (which will happen a few seconds after a client crashes.)

### 05 (tbd)

Currently there is no indication if the client becomes disconnected. Updates for other players stop coming, but that's all.

Instead we should do all this as cleanly as possible. If possible, all clients and the server should quickly become aware of a "game broke" condition and gracefully do whatever, including, on the clients, freezing the simulation and displaying a message.
