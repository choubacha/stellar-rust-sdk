# Contributing to Stellar-sdk

The following is an ever growing set of guidelines for contributing to the stellar sdk.

## Finding work

The progress of the sdk is divided up into several components. We use milestones to track
release targets. We use projects to group related work. And we use issues to track individual
units of work.

#### Issues

Issues are the basic unit of our work and of future PRs. You don't need to work on an issue
to contribute but they are a great way to discover ways to contribute. They will have labels on
them to help you. But feel free to leave comments on any that you either would like to work on
or have questions about.

#### Projects

Projects are used to group and coordinate the completion of issues. They can help provide
additional context around a set of issues and provide some basic ordering to when they should
be completed. They also allow us to think about when a project will be considered _finished_ and
and not just constantly in flight.

#### Milestones

We also like to track when we are going to ship key versions. Patch versions can go out when/as
needed but minor and major versions should be shipped when there are features that require them.
Milestones allow us to track this.

## Testing

We take testing very seriously. Anything that has any logic or complexity should be tested. In
addition, we prefer that examples are documented in doctests so that people that wish to use
this sdk will have code examples that are tested.

We run the tests on Travis currently, and they need to pass in order to merge:
[![Travis](https://img.shields.io/travis/kbacha/stellar-sdk.svg)](https://travis-ci.org/kbacha/stellar-sdk)

#### Doctests

Doctests should provide usage demonstrations and we should hide aspects of the usage that doesn't
provide clarity to users. It's a good idea to provide at least one "meaty" example per struct or
module. This provides solid context to new developers that are just beginning to read and
understand how the SDK works. However, for subsequent function documentation, only a light-weight
example is needed.

#### Unit Tests

Unit tests should test aspects of the code that don't have connections to external resources unless
it can be done simply and without flakiness

#### Integration Tests

These should be added but will likely rely on docker and some sort of internal test server to be 
stood up.

## Documentation

Spend time on your documentation and try to document how someone unfamilar with the library
might approach understanding it. Some ways to make that easier:

* Provide concrete examples on the usage through doc-tests
* Spell check your documentation and use good punctuation
* Empathize with your target audience
  * Think about who you were when you were first starting
  * Think about who you might be in 6 months
  * Think about who you are right now and how you can help yourself understand

The documentation of a library separates good from bad in the rust eco-system. It might feel
like that one last box to tick, but for every new contributor or client, it's the difference
between success and abandon.

#### Reference to horizon

We also emphasize linking back to the horizon documents whenever possible. It's also useful
to copy much of that content over so that it doesn't require external linking but a URL can
go a long way to making it easier to understand the stellar api and eco-sytem itself.

#### Requirements

Since this is a library, all public apis must be documented in order to commit. This helps provide
the expected level of detail to users that wish to install and use our sdk. A build flag should be
set at the header of every crate:

```rust
#![deny(warnings, missing_docs, missing_debug_implementations)]
```

This will cause builds to fail unless all documentation and warnings have been corrected. We also
want debug implementations to be run so that our types can be used in any downstream testing.

## Style

We adhere to rustfmt style. We are using the stable preview version and will continue to do so as
the code we run is on stable. You can find up-to-date installation instructions in the rust-lang
nursery: https://github.com/rust-lang-nursery/rustfmt

Naming and conventions to improve the public interface should be followed and considered. We want
sdk to be as easy to use as possible.

### Traits

When developing a trait please consider that traits are more like verbs than nouns. As such the
convention has been to utilize words like `Clone` instead of `Cloneable` and `Write` instead of
`Writer`. Sometimes a noun might make the right word but generally we should favor describing
what it does rather than what it is.

There is an open proposal for this on the rust-lang nursery:
https://github.com/rust-lang-nursery/api-guidelines/issues/28

### Builder methods

When making a builder method use a `with_` prefix instead of a setter:

```rust

// BAD
fn set_value(mut self, val: Value) -> Self;

// BAD
fn value(mut self, val: Value) -> Self;

// GOOD
fn with_value(mut self, val: Value) -> Self;
```

## Proposing changes

Changes are encouraged and welcome. However, since this is financial software, we must be detailed
in explaining what and why we are making changes.

#### Commits

Commits are just as important to tracking changes as the code itself. As such we highly encourage
you to keep a clean commit history with detailed commit messages. A commit message should not just
describe the code change but provide clarity around why it was changed. It's ok to keep many commits
locally but when submitting your branch for review, please squash all commits into a logical series
of changes.

#### Changelogs

We keep a change log of additions and fixes. Please review how changelogs are kept at the [keepachangelog](https://keepachangelog.com/en/1.0.0/)
website. We have placed a change log in the crate directories of those crates that are released to
crates.io.

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
