//! # petshop_proto
//!
#![recursion_limit = "1024"]
#![type_length_limit = "65536"]
#![deny(missing_debug_implementations)]
#![deny(unused_variables)]
#![warn(clippy::all)]

// This will require proto files to be commented so
// that generated types have documentation
//#![deny(missing_docs)]

/// API module
pub mod api {
    /// Proto definitions
    pub mod v1 {
        tonic::include_proto!("api.v1");
    }
}

/// Google module
pub mod google {
    /// Proto definitions
    pub mod api {
        tonic::include_proto!("google.api");
    }
}
