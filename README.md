# stellar-sdk
A lightweight client SDK for the stellar horizon api. Written in rust.

[![Travis](https://img.shields.io/travis/kbacha/stellar-sdk.svg)](https://travis-ci.org/kbacha/stellar-sdk)

The SDK provides a client and modeled resources to the stellar horizon api. The
resources are strictly typed and parse directly from the api. It also provides a robust
CLI for managing and introspecting the network.

This repository is broken into multiple crates. Each crate is designed to be used
as a different component. The crates are organized as a virtual workspace so that
they can be built together and tested at once.

## Resources

The resources crate documents all the various data types in horizon. These resources
are fully documented and can be deserialized from the client.

## Client

The client provides two interfaces. One is the synchronous client and the other is
the asynchronous client. Both consume an EndPoint trait and will return the appropriate
response associated with the endpoint implementation. You should see the documentation
associated with the code itself but below is a brief example

```rust
use stellar_client::sync::Client;
use stellar_client::endpoint::asset;

let client      = Client::horizon_test().unwrap();
let endpoint    = asset::All::default();
let records     = client.request(endpoint).unwrap();
```

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

We welcome contributors! Please see our [contributing](https://github.com/kbacha/stellar-sdk/blob/master/CONTRIBUTING.md) guide for information
on how to contribute to our repo!

## TODO

- [x] Implement all resources
- [x] Synchronous client
- [ ] Asynchronous client
- [ ] Implement all endpoints to horizon
- [ ] Implement complete CLI to request data from horizon
- [ ] Implement informational CLI
- [ ] Implement XDR resources for stellar
- [ ] Implement ability to create transactions and submit to network
