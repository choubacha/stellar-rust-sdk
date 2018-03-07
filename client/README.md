# Stellar Client

A crate that enables interacting with the stellar horizon api.

### Endpoints

An endpoint is a trait that embodies the two way nature of an API. On the
one hand it deines what is returned from the API as strict types but it also
defines what needs to be specified to make the requests in the first place.

Having this captured in a type allows us to reason about it in the abstract
and model the abstraction primitives going into and out of horizon.
