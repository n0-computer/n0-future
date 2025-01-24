//! Re-exports of abstractions and implementations deemed useful and good by number 0 engineers.
//!
//! Read up more on our challenges with rust's async: <https://www.iroh.computer/blog/async-rust-challenges-in-iroh>
//!
//! This library also allows importing a single [`task`] and [`time`] module that'll work
//! in `wasm*-*-unknown` targets, using `wasm_bindgen` and `wasm_bindgen_futures`, but mirroring
//! the `tokio` API with only minor differences.

#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![cfg_attr(n0_future_docsrs, feature(doc_auto_cfg))]

pub mod task;
pub mod time;
