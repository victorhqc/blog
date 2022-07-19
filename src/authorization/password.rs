use data_encoding::HEXUPPER;
use ring::{digest, pbkdf2};
use snafu::prelude::*;
use std::env;
use std::num::NonZeroU32;

pub fn encrypt_password(password: &str) -> Result<String, EnctyptionError> {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY is not defined");
    let salt = secret.as_bytes();

    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );

    Ok(HEXUPPER.encode(&pbkdf2_hash))
}

pub fn verify_password(password: &str, known_hash: &str) -> Result<bool, EnctyptionError> {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY is not defined");
    let salt = secret.as_bytes();

    let n_iter = NonZeroU32::new(100_000).unwrap();

    let verification = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        salt,
        password.as_bytes(),
        &HEXUPPER.decode(known_hash.as_bytes()).unwrap(),
    );

    Ok(verification.is_ok())
}

#[derive(Debug, Snafu)]
#[snafu(display("Encryption failed"))]
pub struct EnctyptionError;

impl From<ring::error::Unspecified> for EnctyptionError {
    fn from(_: ring::error::Unspecified) -> Self {
        EnctyptionError
    }
}
