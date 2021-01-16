use rusoto_core::RusotoError;
pub mod common;

use div_cloud::auth::cognito::CognitoClient;
use div_cloud::auth::types::*;


pub fn get_cognito_idp_client() -> CognitoClient {
    CognitoClient::new()
}

pub fn gets_client_ok() -> Result<CognitoClient, String> {
    let client = CognitoClient::new();
    Ok(client)
}

#[tokio::test]
async fn get_user_ok() -> Result<(), String> {
    Ok(())
}


#[tokio::test]
async fn create_user_ok() -> Result<(), String> {
    Ok(())
}

#[tokio::test]
async fn deletes_user_ok() -> Result<(), String> {
    Ok(())
}

#[tokio::test]
async fn signs_up_user_ok() -> Result<(), String> {
    Ok(())
}

#[tokio::test]
async fn signs_out_user_ok() -> Result<(), String> {
    Ok(())
}

#[tokio::test]
async fn confirms_signup_ok() -> Result<(), String> {
    Ok(())
}

#[tokio::test]
async fn lists_users_ok() -> Result<(), String> {
    Ok(())
}

#[tokio::test]
async fn lists_user_pools_ok() -> Result<(), String> {
    Ok(())
}
