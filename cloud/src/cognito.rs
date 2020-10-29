pub mod types;

use rusoto_core::{Region, RusotoResult, RusotoError, request::BufferedHttpResponse};

use crate::cognito::types::{CgUser, CgAuthRes, CgDeviceMeta, CgSignupRes, Challenge, CgUserSignup, CgUserLogin};
use std::collections::HashMap;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};
use rusoto_cognito_idp::{ AttributeType, NewDeviceMetadataType, UserPoolDescriptionType,
    GlobalSignOutResponse, InitiateAuthError, 
    AdminGetUserRequest, AdminGetUserResponse, AdminGetUserError, AuthenticationResultType,
    AdminInitiateAuthRequest, AdminInitiateAuthResponse,
    AdminConfirmSignUpRequest, AdminConfirmSignUpResponse, AdminConfirmSignUpError, 
    AdminDeleteUserError, AdminDeleteUserRequest,
    AdminCreateUserError, AdminCreateUserRequest, AdminCreateUserResponse,
    CognitoIdentityProviderClient, CognitoIdentityProvider,
    SignUpRequest, ConfirmSignUpRequest, SignUpError, SignUpResponse,
    ListUsersRequest, ListUsersResponse, ListUsersError, 
    ListUserPoolsRequest, ListUserPoolsResponse, ListUserPoolsError, 
    GlobalSignOutRequest,  GlobalSignOutError, InitiateAuthRequest,
};
use rusoto_cognito_identity::{
    SetIdentityPoolRolesInput, GetIdInput, Credentials, IdentityPool, GetOpenIdTokenInput,
    CognitoIdentity, GetCredentialsForIdentityInput, GetCredentialsForIdentityResponse,
    CognitoIdentityClient, 
    ListIdentitiesInput, 
    ListIdentityPoolsInput, 
    GetOpenIdTokenForDeveloperIdentityInput, 
    CreateIdentityPoolInput, 
    DeleteIdentityPoolInput, 
    CognitoProvider
};

fn get_user_pool_id() -> String { 
    dotenv::var("AWS_COGNITO_USER_POOL_ID").expect("Cognito user pool ID not set")
}

fn get_client_id(di_srv: bool) -> String {
    if di_srv {
        dotenv::var("AWS_COGNITO_CLIENT_ID_DI_SRV").expect("No client ID set")
    } else {
        dotenv::var("AWS_COGNITO_CLIENT_ID").expect("No client ID set")
    }
}

fn get_client_secret() -> String {
    dotenv::var("AWS_COGNITO_CLIENT_SECRET").expect("No client secret set")
}

#[derive(Clone)]
pub struct CognitoClient {
    id: CognitoIdentityClient,
    idp: CognitoIdentityProviderClient,
}

impl CognitoClient {

    pub fn new() -> Self {
        let idp = CognitoIdentityProviderClient::new(Region::UsWest2);
        let id = CognitoIdentityClient::new(Region::UsWest2);
        Self { idp, id}
    }

    pub async fn get_user(&self, username: &str) ->  Result<CgUser, AdminGetUserError> 
    {
        let req = AdminGetUserRequest {
            username: username.to_string(), user_pool_id: get_user_pool_id(),
        };
        match self.idp.admin_get_user(req).await {
            Ok(user) =>  Ok(CgUser::from(user)),
            Err(_err) => { Err(AdminGetUserError::UserNotFound("Not found".to_string())) }
        }
    }

    pub async fn login_user(&self, user: CgUserLogin)
        ->  Result<CgAuthRes, String> 
    {
        let mut params = HashMap::new();
        params.insert("USERNAME".to_string(), user.username);
        params.insert("PASSWORD".to_string(), user.password);
        //params.insert("SECRET_HASH".to_string(), get_client_secret());
        let req = InitiateAuthRequest {
            auth_flow: "USER_PASSWORD_AUTH".to_string(),
            client_id: get_client_id(true),
            auth_parameters: Some(params),
            ..Default::default()
        };
        match self.idp.initiate_auth(req).await {
            Ok(resp) => match resp.authentication_result {
                Some(res) => { return Ok(CgAuthRes::from(res)); },
                None => Err("".to_string()),
            },
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn create_user(&self, 
        user: CgUserSignup, email_verified: bool) -> Result<CgUser, String> 
    {
        let attrib = vec![
            AttributeType { 
                name: "email".into(), 
                value: Some(user.email) 
            },
            AttributeType { 
                name: "email_verified".into(), 
                value: Some(email_verified.to_string()),
            }
        ];
        let req = AdminCreateUserRequest {
            username: user.username,
            temporary_password: Some(user.password),
            desired_delivery_mediums: Some(vec!["EMAIL".to_string()]),
            validation_data: None,
            user_attributes: Some(attrib),
            user_pool_id: get_user_pool_id(),
            client_metadata: None,
            ..Default::default()
        };
        match self.idp.admin_create_user(req).await {
            Ok(res) => Ok(CgUser::from(res.user.unwrap())),
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn confirm_signup(&self, username: String) -> Result<String, String> {
        let req = AdminConfirmSignUpRequest {
            username: username.clone(), user_pool_id: get_user_pool_id(),
            client_metadata: None,
        };
        match self.idp.admin_confirm_sign_up(req).await {
            Ok(_res) => Ok(format!("Confirmed user {:?}", &username)),
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn delete_user(&self, username: String) -> Result<String, String> {
        let req = AdminDeleteUserRequest {
            username, user_pool_id: get_user_pool_id(),
        };
        match self.idp.admin_delete_user(req).await {
            Ok(res) => Ok("Deleted user".to_string()),
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn signout_user(&self, access_token: String) -> Result<String, String> {
        let req = GlobalSignOutRequest { access_token };
        match self.idp.global_sign_out(req).await {
            Ok(res) => Ok("Signed out".to_string()),
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn list_user_pools(&self) -> Result<Vec<UserPoolDescriptionType>, String> 
    {
        let req = ListUserPoolsRequest { max_results: 5, next_token: None };
        match self.idp.list_user_pools(req).await {
            Ok(res) => Ok(res.user_pools.unwrap()),
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn list_users(&self) -> () {}
    pub async fn validate_token(&self) -> () {}
    pub async fn validate_session(&self) -> () {}
    pub async fn update_user_attrib(&self) -> () {}
    pub async fn confirm_email(&self) -> () {}

    pub async fn signup_user(&self, user: CgUserSignup)
        ->  RusotoResult<CgSignupRes, SignUpError>
    {
        let attrib = Some(vec![
            AttributeType { 
                name: "email".into(), 
                value: Some(user.email) 
            },
        ]);
        let req = SignUpRequest {
            client_id: get_client_id(true),
            //secret_hash: Some(get_client_secret()),
            username: user.username, 
            password: user.password, 
            user_attributes: attrib,
            ..Default::default()
        };
        match self.idp.sign_up(req).await {
            Ok(resp) => Ok(CgSignupRes::from(resp)),
            Err(err) => Err(err)
        }
    }

    pub async fn get_credentials_for_id(id: String) -> Credentials {
       Credentials::default() 
    }
}

fn get_provider() -> CognitoProvider {
    let region = Region::UsWest2;
    let provider = CognitoProvider::builder()
         .region(region.clone())
         .build();
    provider
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn list_user_pools_ok() {
        let client = CognitoClient::new();
        let pools =  client.list_user_pools().await;
        debug_assert!(pools.is_ok())
    }

    #[tokio::test]
    pub async fn create_user_ok() {
        let client = CognitoClient::new();
        let pools =  client.list_user_pools().await;
        assert!(pools.is_ok()) 
    }

    #[tokio::test]
    pub async fn signup_user_ok() {
        let client = CognitoClient::new();
        let pools =  client.list_user_pools().await;
        assert!(pools.is_ok()) 
    }
}
