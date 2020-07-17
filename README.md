# ranged_type

An (early-stage) crate providing ranged numeric types.

Ranged types are a great way to leverage the type system to provide more expressive typestate.  It's
a good rule of thumb to prefer compile-time errors over runtime errors.

Ranged types in this crate are designed such that arithmetic of two ranged types is a zero runtime 
overhead (ZRO) operation.

## Current Limitations
* Early stage PoC implementation
* Range bounds must be `const`.  May be possible in the future to relax this to also permit runtime
values as well (such values would no longer benefit from ZRO arithmetic operations, of course).
* Currently depends on many unstable features to leverage typestate and achieve ZRO goals.
* Publishing as a crate at this early stage to (hopefully) prevent inadvertent future breakage such as
the loss of compile-time `const` arithmetic on `rustc nightly-2020-06-04`.  

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.
