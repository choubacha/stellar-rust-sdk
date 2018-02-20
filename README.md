# stellar-sdk
A lightweight asynchronous SDK for the stellar horizon api. Written in rust.

[![Travis](https://img.shields.io/travis/kbacha/stellar-sdk.svg)](https://travis-ci.org/kbacha/stellar-sdk)

## Directory structure

This repository is broken into multiple crates. Each crate is designed to be used
as a different component. The crates are organized as a virtual workspace so that
they can be built together and tested at once.

## Testing

From the main (virtual workspace) run the following command. It should run all the
tests (including doctests) across the entire repo.

```bash
cargo test
```
