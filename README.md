# chat-rs

This is a chat application client that uses gRPC for communication. It is written to experiment with streaming RPCs and gRPC in general.

## Running the client

The client can be built using the following command:

```bash
cargo build
```

The client can be run using the following command:

```bash
cargo run --bin chat-client
```

## Running the server

This repo also includes a server implementation written in rust but is incomplete at the time of writing.

The server can be run using the following command:

```bash
cargo run --bin chat-server
```