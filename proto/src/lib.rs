//! # petshop_proto
//!
#![recursion_limit = "1024"]
#![type_length_limit = "65536"]
#![deny(missing_debug_implementations)]
#![deny(unused_variables)]
#![warn(clippy::all)]

// FIXME: Enable this to require proto documentation
// #![deny(missing_docs)]

tonic::include_proto!("api.v1");
