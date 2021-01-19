use div_api::auth::PwVerifier;

#[test]
fn it_hashes_password_ok() -> argon2::Result<String> {
    let ver = PwVerifier::new();
    let pw = "password";
    ver.hash(pw)
}

#[test]
fn it_verifies_hashed_password() -> argon2::Result<()> {
    let ver = PwVerifier::new();
    let pw = "password";
    let hash = ver.hash(pw)?;
    debug_assert_eq!(true, ver.verify(pw, hash)?);
    Ok(())
}
