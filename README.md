# cloudflare-rs
> Rust library for the Cloudflare v4 API

[![Cloudflare's crates.io badge](https://img.shields.io/crates/v/cloudflare.svg)](https://crates.io/crates/cloudflare)
[![Cloudflare's docs.rs badge](https://docs.rs/cloudflare/badge.svg)](https://docs.rs/cloudflare)

⚠️ This library is a Work in Progress! ⚠️

This library provides convenience functions that wrap the Cloudflare API.

It provides two client implementations (asynchronous vs blocking). However, 
projects targeting `wasm32` only get the asynchronous one (as it does not make
sense to block in that target).

## Fork

This is a fork of the `cloudflare` crate.
I've tried upstreaming my patches but they have not yet been merged and release. 

This might not grow beyond my personal fork with few bug fixes but I do intend to
make some architectural changes that some might find useful.
