# Functors and Monads in Rust

This crate provides functors and monads in Rust.

## History

Up to version `0.8.x`, the API was designed to allow additional bounds on inner
types, thus allowing implementation of a common `Functor` trait for all
collections in
[`std::collections`](https://doc.rust-lang.org/std/collections/), including
those which have an `Eq + Hash` or an `Ord` bound on their relevant methods.
This support has been dropped as of version `0.9.0` due to problems with
required bounds when using these traits.

## Current state

Current versions of `fmap` only support functors that are also monads. This is
done through a `Monad` trait, which also provides a `Monad::fmap` function. The
simplified approach solves issues with trait bounds.
