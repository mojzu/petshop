fn main() {
    tonic_build::configure()
        // FIXME: Derive Validate trait and add validation to fields here
        //
        // It would be nicer to define these somewhere in the proto file, but that seems unlikely to be
        // supported in the near future
        //
        // Validator functions have to be wrapped to make them usable with the prost types, an alternative
        // way of doing this might be to implement the Validate trait manually on structs in lib.rs
        //
        // <https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html>
        // <https://cheatsheetseries.owasp.org/cheatsheets/REST_Security_Cheat_Sheet.html#input-validation>
        .type_attribute("api.User", "#[derive(Validate)]")
        .field_attribute(
            "api.User.email",
            "#[validate(custom = \"prost_validator::email\")]",
        )
        .field_attribute(
            "api.User.name",
            "#[validate(custom = \"prost_validator::user_name\")]",
        )
        .compile(&["proto/api.proto"], &["proto"])
        .expect("tonic_build failed");
}
