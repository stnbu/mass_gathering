# I describe to myself how `bevy-renet` works

## Client

To send a message to the server:

```rust
pub fn send_message(
    mut client: ResMut<RenetClient>,
) {
    let message = bincode::serialize("whatever").unwrap();
    client.send_message(CHANNEL, message);
}
```

To relay local events to the server, a client can have a system like this:

```rust
pub fn relay_local_events_to_server(
    mut events: EventReader<SomeEvent>,
    mut client: ResMut<RenetClient>,
) {
    for event in events.iter() {
        let message = bincode::serialize(event).unwrap();
        client.send_message(CHANNEL, message);
    }
}
```

Messages from server are read using the following pattern:

```rust
pub fn receive_from_server(mut client: ResMut<RenetClient>) {
    while let Some(bytes) = client.receive_message(CHANNEL) {
        let server_message = bincode::deserialize(&bytes).unwrap();
        match server_message {
            _ => {
                println!("{server_message:?}")
            }
        }
    }
}
```

## Server

`ServerEvent`s are part of the core Renet infrastructure. They are not things that _you_
explicitly send. There are only _two_ of them. They happen automatically upon connect and
disconnect and therefore only the server needs code to deal with them.

```rust
pub fn receive_server_events(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, user_data) => {}
            ServerEvent::ClientDisconnected(id) => {}
        }
    }
}
```

To receive messages from clients:

```rust
pub fn receive_from_client(mut server: ResMut<RenetServer>) {
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, CHANNEL) {
            let message = bincode::deserialize(&message).unwrap();
            match message {}
        }
    }
}
```

To send messages to clients:

```rust
pub fn send_messages_to_clients(
    mut server: ResMut<RenetServer>,
) {
    let client_id = 1234;
    let message = bincode::serialize("whatever").unwrap();
    server.send_message(client_id, CHANNEL, message);
    let message = bincode::serialize("whateverybody").unwrap();
    server.broadcast_message(DefaultChannel::Reliable, message);
}
```

## Creating the client/server resources

```rust
pub fn get_server() -> RenetServer {
    let server_addr = "127.0.0.1:8081".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config =
        ServerConfig::new(64, 1234567890, server_addr, ServerAuthentication::Unsecure);
    RenetServer::new(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        server_config,
        RenetConnectionConfig::default(),
        socket,
    )
    .unwrap()
}
```

```rust
pub fn get_client(client_id: u64) -> RenetClient {
    let server_addr = "127.0.0.1:8081".parse().unwrap();
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    RenetClient::new(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        socket,
        RenetConnectionConfig::default(),
        authentication,
    )
    .unwrap()
}
```

Note that they are irritatingly similar and yet there's not much duplicate code to factor out.

Note that there is no such thing as a "ClientConfig".

Since these return a bevy `Resource` we use these thusly:

```rust
my_app.insert_resource(get_server())
```

The client will start trying to connect as soon as the `RenetClient` resource comes into existence (the function returns).
