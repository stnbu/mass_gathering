# Here I claim to list important concepts with regard to this project's organization (mostly from the `cargo` point of view).

Heading depth represents directory depth.

## src/

Obviously it all starts here.

### src/lib.rs

This is the whole Mass Gathering crate. We may consider keeping this file as simple as
possible (roughly: `pub use` and `pub mod` declarations.)

### src/events.rs

Event type-code. The types and code related to their behavior.

### src/server.rs

Strictly server library code.

### src/client.rs

Strictly client library code.

### src/bin/

The two binaries. Maybe `/mod.rs` style, if that's possible and the contents get
complicated (but why would they?)

#### src/bin/client.rs

#### src/bin/server.rs
