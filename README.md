# Functors (and Monads) in Rust

This crate provides functors and monads in Rust. The API is designed to
allow additional bounds on inner types, thus allowing implementation of
a common `Functor` trait for all collections in
[`std::collections`](https://doc.rust-lang.org/std/collections/),
including those which have an `Eq + Hash` or an `Ord` bound on their
relevant methods.

Also included in this crate are `Functor` and `Monad` implementations
for boxed iterators, futures, and functions.
