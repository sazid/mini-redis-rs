# Mini Redis Rust

A very simple implementation of a Redis like server in Rust - for learning
purposes (specially the [Tokio](https://tokio.rs) stack).

## Notes:

- Try to never use `unwrap()` in production, if you so require, try using
  `expect()`.
- In Tokio, a **task** is the equivalent of terms like green thread, fiber,
  coroutine, etc.
- Rust `async fn` always returns an anonymous `impl Future` type.
- Rust async is **lazy** in nature. Unless `.await` is called, a `Future` will
  never be executed.
- Use `type` aliases to clarify and shorten; long types.
- Sharding is a very basic but awesome concept - allows you to split your data
  storage into multiple parts (or shards) to reduce contentions for both reads
  and writes. Check [lib.rs](src/lib.rs) for a basic implementation.
- Use `tokio::sync::oneshot` instead of callbacks for passing messages around
  and communicating among different tasks.
- Pair `tokio::sync::mpsc` with **task**s to create manager tasks that manage
  communication with services such as db, api clients, etc. See
  [client.rs](src/bin/client.rs) for an example.

## Run

Run the server in one terminal.

```sh
cargo run --bin server
```

Run the client in another terminal.

```sh
cargo run --bin client
```
