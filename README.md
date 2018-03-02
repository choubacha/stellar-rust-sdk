# stellar-sdk
A lightweight asynchronous SDK for the stellar horizon api. Written in rust.

[![Travis](https://img.shields.io/travis/kbacha/stellar-sdk.svg)](https://travis-ci.org/kbacha/stellar-sdk)

## Directory structure

This repository is broken into multiple crates. Each crate is designed to be used
as a different component. The crates are organized as a virtual workspace so that
they can be built together and tested at once.

### Resources

Defines the major data types that constitute the horizon API resources. It also enables
the serialization to and from JSON.

### Client

Defines a connection to a horizon server and manages the interactions to and from it.
This includes creating new resources and fetching existing resources.

## Testing

From the main (virtual workspace) run the following command. It should run all the
tests (including doctests) across the entire repo.

```bash
cargo test
```
