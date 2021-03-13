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
    use crate::prost_validator;
    tonic::include_proto!("api");
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
        if validator::validate_length(s, None, Some(128), None) {
            Ok(())
        } else {
            Err(ValidationError::new("user_name_invalid"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::api::*;
    use validator::Validate;

    // FIXME: Unit test example, can also use doctests here but not in the
    // server crate (because it's a binary), worth splitting server
    // functionality into a library?

    #[test]
    fn user_validate_test() {
        let user = User {
            email: "validemail@example.com".to_string(),
            name: "validname".to_string(),
        };
        assert_eq!(user.validate().is_ok(), true);

        let user = User {
            email: "notanemail".to_string(),
            name: "validname".to_string(),
        };
        assert_eq!(user.validate().is_ok(), false);

        let user = User {
            email: "validemail@example.com".to_string(),
            name: "v".to_string(),
        };
        assert_eq!(user.validate().is_ok(), false);
    }
}
