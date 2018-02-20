# Contributing to Stellar-sdk

The following is an ever growing set of guidelines for contributing to the stellar sdk.

### Testing

We take testing very seriously. Anything that has any logic or complexity should be tested. In
addition, we prefer that examples are documented in doctests so that people that wish to use
this sdk will have code examples that are tested.

We run the tests on Travis currently, and they need to pass in order to merge:
[![Travis](https://img.shields.io/travis/kbacha/stellar-sdk.svg)](https://travis-ci.org/kbacha/stellar-sdk)

#### Doctests

Doctests should provide usage demonstrations and we should hide aspects of the usage that doesn't
provide clarity to users.

#### Unit Tests

Unit tests should test aspects of the code that don't have connections to external resources unless
it can be done simply and without flakiness

#### Integration Tests

These should be added but will likely rely on docker and some sort of internal test server to be 
stood up.

### Documentation

Since this is a library, all public apis must be documented in order to commit. This helps provide
the expected level of detail to users that wish to install and use our sdk. A build flag should be
set at the header of every crate:

```rust
#![deny(warnings, missing_docs, missing_debug_implementations)]
```

This will cause builds to fail unless all documentation and warnings have been corrected. We also
want debug implementations to be run so that our types can be used in any downstream testing.

### Style

We adhere to rustfmt style. You can find up-to-date installation instructions in the rust-lang
nursery: https://github.com/rust-lang-nursery/rustfmt

Naming and conventions to improve the public interface should be followed and considered. we want
sdk to be as easy to use as possible.


### Proposing changes

Changes are encouraged and welcome. However, since this is financial software, we must be detailed
in explaining what and why we are making changes.

#### Commits

Commits are just as important to tracking changes as the code itself. As such we highly encourage
you to keep a clean commit history with detailed commit messages. A commit message should not just
describe the code change but provide clarity around why it was changed. It's ok to keep many commits
locally but when submitting your branch for review, please squash all commits into a logical series
of changes.

#### Pull Requests

All submittals should be provided via PR. The PRs should be detailed and describe the intent of the
change as well as possible. "Intent" does not mean a description of the code changes by itself, it
means describing the reasoning behind the change.

#### Merging

We use a rebase-and-merge strategy and thus wish for all commits to be clean and reviewed before
merging. In addition we ask that the branch be up-to-date before merging, in case any changes
might break upon being merged.

## Thanks!

And lastly, thanks for taking the time to contribute!
