# WebAssembly, Automatically Abstracted Away

This crate is a collection of small helpers, designed to help writing crates that work on both the web platform and native.

It does not contain any implementation code, and only delegates to production-ready implementations.

Its main value is abstracting away the different implementation types, so that you do not have to care about whether your code is running on the web or on native.

In particular, it has a whole set of `Send`- and `Sync`-related traits. These traits are the thread-safe variant for native, and on the web they will fallback to the non-thread-safe variant. This is because `JsValue`s are not thread-safe, and `wasm-bindgen-futures`'s executor is thread-local anyway, so there is much less need for thread-safety.

This being said, this crate is opinionated. What it offers may not necessarily match what you need. If you think of a missing feature that could be abstracted away between web and native applications please open a PR, but eg. if you need a thread-safe future on the web then you should just use `Send`.

This crate is very much work-in-progress. While everything implemented should be working fine (because there is basically no code at all, it only redirects to well-known implementations), there are certainly lots of features that are not implemented yet. Pull requests are welcome!
