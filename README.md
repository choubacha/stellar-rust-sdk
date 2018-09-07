# stellar-rust-sdk
A lightweight client SDK for the stellar horizon api. Written in rust.

[![Travis](https://img.shields.io/travis/kbacha/stellar-rust-sdk.svg)](https://travis-ci.org/kbacha/stellar-rust-sdk)
[![Gitter](https://img.shields.io/gitter/room/nwjs/nw.js.svg)](https://gitter.im/stellar-rust-sdk)
[![Crates.io](https://img.shields.io/crates/v/stellar-client.svg)](https://crates.io/crates/stellar-client)
[![Docs.rs](https://docs.rs/stellar-client/badge.svg)](https://docs.rs/stellar-client/)

The SDK provides a client and modeled resources to the stellar horizon api. The
resources are strictly typed and parse directly from the api. It also provides a robust
CLI for managing and introspecting the network.

This repository is broken into multiple crates. Each crate is designed to be used
as a different component. The crates are organized as a virtual workspace so that
they can be built together and tested at once.

## Client

The client provides two interfaces. One is the synchronous client and the other is
the asynchronous client. Both consume an IntoRequest trait and will return the appropriate
response associated with the endpoint implementation. You should see the documentation
associated with the code itself since there are active doctests available for all endpoints.
However, the general gist is that the developer should create an endpoint and then exchange
that endpoint for a response from the client:

```rust
use stellar_client::sync::Client;
use stellar_client::endpoint::asset;

let client      = Client::horizon_test().unwrap();
let endpoint    = asset::All::default();
let records     = client.request(endpoint).unwrap();
```

### Endpoints

The endpoints module in the client crate houses all the horizon API endpoint definitions. Each
struct will define what parameters are required for making requests.

### Resources
The resources module contains the return values of the various endpoints. These resources are
fully documented and can be deserialized from the client. There are many resources
in the horizon API and you can read about all of them [here](https://www.stellar.org/developers/horizon/reference/index.html).

They are deserialized into types that we think will make using them inside other rust applications
easier.

## CLI

The CLI is a command line utility for querying against the stellar network. To install it
clone the repo and run:

```
cargo install -f --path=cli
```

You can then use it with:

```
stellar --help
```

The CLI is most useful for easily perusing horizon/stellar data without needing to construct
curl commands. It is also a working example of how the stellar SDK can be consumed and used.

## Testing

Setup is managed through cargo. We develop on stable but you should be up-to-date before
running tests.

```
cargo test
```

## Documentation

To build the documentation locally, just use the doc command:

```
cargo doc
```

To build and automatically open the client documentation:

```
cargo doc -p stellar-client --open
```

To build and automatically open the resource documentation:

```
cargo doc -p stellar-resources --open
```

## Contributing

We welcome contributors! Please see our [contributing](https://github.com/kbacha/stellar-rust-sdk/blob/master/CONTRIBUTING.md) guide for information
on how to contribute to our repo!

## TODO

- [x] Implement all resources
- [x] Synchronous client
- [ ] Asynchronous client (waiting stabilization of hyper)
- [ ] Implement all endpoints to horizon
- [ ] Implement complete CLI to request data from horizon
- [ ] Implement informational CLI
- [ ] Implement XDR resources for stellar
- [ ] Implement ability to create transactions and submit to network
