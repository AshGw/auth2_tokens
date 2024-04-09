use crate::consts::{CV_DEFAULT_SIZE, CV_MAX_SIZE, CV_MIN_SIZE};
use crate::urlsafe::{urlsafe_b64encode, urlsafe_token};
use sha2::{Digest, Sha256};

/// Generates a random [code verifier](https://datatracker.ietf.org/doc/html/rfc7636#section-4.1)
/// string of a specified size.
///
/// ## Arguments:
///
/// `n`: The number of characters to generate.
/// <br>
/// ### PANICS !
/// if not between `43` & `128`. Defaults to `96` characters if no size is provided.
pub fn gen_code_verifier(n: Option<usize>) -> String {
    let size: usize = n.unwrap_or(CV_DEFAULT_SIZE);
    if !(CV_MIN_SIZE..=CV_MAX_SIZE).contains(&size) {
        panic!(
            "Invalid size, the size must be between {} and {}",
            CV_MIN_SIZE, CV_MAX_SIZE
        );
    }
    urlsafe_token(size)
}

/// Generates a URL safe base64 encoded string from the code verifier.
///
/// ## Arguments:
///
/// `code_verifier`:  a random string that is generated by the client and used to create a code challenge.
pub fn gen_code_challenge(code_verifier: &str) -> String {
    urlsafe_b64encode(Sha256::digest(code_verifier))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_code_verifier_default_size() {
        let code_verifier: String = gen_code_verifier(None);
        assert_eq!(code_verifier.len(), CV_DEFAULT_SIZE);
    }

    #[test]
    fn test_gen_code_verifier_custom_size() {
        let custom_size: usize = 64;
        let code_verifier: String = gen_code_verifier(Some(custom_size));
        assert_eq!(code_verifier.len(), custom_size);
    }

    #[test]
    #[should_panic]
    fn test_gen_code_verifier_invalid_size() {
        gen_code_verifier(Some(32));
    }

    #[test]
    fn test_gen_code_challenge() {
        let code_verifier: &str = "foo_bar";
        let code_challenge: String = gen_code_challenge(code_verifier);
        assert_eq!(code_challenge.len(), CV_MIN_SIZE);
    }
}
