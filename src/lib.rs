use std::borrow::Borrow;

/// The "credentials" pair defined in [RFC 5849 section 1.1][rfc].
///
/// [rfc]: https://tools.ietf.org/html/rfc5849#section-1.1
///
/// This type represents:
///
/// - Client credentials (consumer key and secrets)
/// - Temporary credentials (request token and secret)
/// - Token credentials (access token and secret)
#[derive(Clone, Copy, Debug)]
pub struct Credentials<T = String> {
    pub identifier: T,
    pub secret: T,
}

impl<T: Borrow<str>> Credentials<T> {
    pub fn new(identifier: T, secret: T) -> Self {
        Credentials { identifier, secret }
    }

    pub fn identifier(&self) -> &str {
        self.identifier.borrow()
    }

    pub fn secret(&self) -> &str {
        self.secret.borrow()
    }

    pub fn as_ref(&self) -> Credentials<&str> {
        Credentials {
            identifier: self.identifier.borrow(),
            secret: self.secret.borrow(),
        }
    }
}
