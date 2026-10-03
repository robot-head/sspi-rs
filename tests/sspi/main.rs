#![allow(clippy::panic, reason = "panics are acceptable in test code")]

mod client_server;
mod common;
mod ntlm;

#[test]
fn invalid_mechanism_oid_is_an_invalid_token() {
    let cause = picky_krb::gss_api::GssApiMessageError::InvalidMechanismOid(
        "1.2.840.113554.1.2.2".into(),
        "1.3.6.1.4.1.311.2.2.10".into(),
    );
    let description = cause.to_string();
    let error = sspi::Error::from(cause);
    assert_eq!(error.error_type, sspi::ErrorKind::InvalidToken);
    assert_eq!(error.description, description);
}
