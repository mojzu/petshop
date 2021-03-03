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

#[macro_use]
extern crate validator;

/// API module
pub mod api {
    /// Proto definitions
    pub mod v1 {
        use crate::prost_validator;
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

/// Prost wrappers for validator library
///
/// See `build.rs` file for adding these to prost message fields
pub mod prost_validator {
    use validator::ValidationError;

    pub fn email(s: &str) -> Result<(), ValidationError> {
        if validator::validate_email(s) {
            Ok(())
        } else {
            Err(ValidationError::new("email_invalid"))
        }
    }

    pub fn user_name(s: &str) -> Result<(), ValidationError> {
        if validator::validate_length(s, Some(2), Some(500), None) {
            Ok(())
        } else {
            Err(ValidationError::new("user_name_invalid"))
        }
    }
}
