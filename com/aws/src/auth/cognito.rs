use rusoto_core::{Region, RusotoResult, RusotoError, request::BufferedHttpResponse};

use super::types::{CgUser, CgAuthRes, CgDeviceMeta, CgSignupRes, Challenge};
use std::collections::HashMap;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};
use rusoto_cognito_idp::{ AttributeType, NewDeviceMetadataType, UserPoolDescriptionType,
    AdminGetUserRequest, AdminGetUserResponse, AdminGetUserError, AuthenticationResultType,
    AdminInitiateAuthError, AdminInitiateAuthRequest, AdminInitiateAuthResponse,
    AdminCreateUserError, AdminCreateUserRequest, AdminCreateUserResponse,
    CognitoIdentityProviderClient, CognitoIdentityProvider,
    SignUpRequest, ConfirmSignUpRequest, SignUpError, SignUpResponse,
    AdminConfirmSignUpResponse, AdminConfirmSignUpError,
    ListUsersRequest, ListUsersResponse, ListUsersError, 
    ListUserPoolsRequest, ListUserPoolsResponse, ListUserPoolsError, 
    GlobalSignOutRequest, GlobalSignOutResponse,
    GlobalSignOutError,
    AdminConfirmSignUpRequest, InitiateAuthRequest,
};
use rusoto_cognito_identity::{
    SetIdentityPoolRolesInput, GetIdInput, Credentials, IdentityPool, GetOpenIdTokenInput,
    CognitoIdentity, 
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

fn get_client_id() -> String {
    dotenv::var("AWS_COGNITO_CLIENT_ID").expect("No client ID set")
}

fn get_client_secret() -> String {
    dotenv::var("AWS_COGNITO_CLIENT_SECRET").expect("No client secret set")
}

fn get_id_client() -> CognitoIdentityClient {
    let client = CognitoIdentityClient::new(Region::UsWest2);
    client
}

fn get_idp_client() -> CognitoIdentityProviderClient {
    let client = CognitoIdentityProviderClient::new(Region::UsWest2);
    client
}

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

    pub async fn get_user(&self,
        username: String) ->  Result<CgUser, AdminGetUserError> 
    {
        let req = AdminGetUserRequest {
            username, user_pool_id: get_user_pool_id(),
        };
        match self.idp.admin_get_user(req).await {
            Ok(user) =>  Ok(CgUser::from(user)),
            Err(_err) => { Err(AdminGetUserError::UserNotFound("Not found".to_string())) }
        }
    }

    pub async fn login_user(&self, username: String, password: String) 
        ->  Result<CgAuthRes, AdminInitiateAuthError> 
    {
        let mut params = HashMap::new();
        params.insert("USERNAME".to_string(), username);
        params.insert("PASSWORD".to_string(), password);
        params.insert("SECRET_HASH".to_string(), get_client_secret());
        let req = AdminInitiateAuthRequest {
            auth_flow: "USERPASSWORDAUTH".to_string(),
            client_id: get_client_id(),
            user_pool_id: get_user_pool_id(),
            auth_parameters: Some(params),
            ..Default::default()
        };
        match self.idp.admin_initiate_auth(req).await {
            Ok(resp) => match resp.authentication_result {
                Some(res) => { return Ok(CgAuthRes::from(res)); },
                None => {
                    return Err(
                        AdminInitiateAuthError::
                        NotAuthorized("Couldnt authorize".to_string())); 
                }
            },
            Err(err) => Err(
                AdminInitiateAuthError::
                NotAuthorized("Couldnt authorize".to_string()))
                
        }
    }

    pub async fn create_user(&self, username: String) -> () {}

    pub async fn signout_user(&self, access_token: String) -> Result<String, String> {
        let req = GlobalSignOutRequest { access_token };
        match self.idp.global_sign_out(req).await {
            Ok(res) => Ok("Signed out".to_string()),
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn list_user_pools(&self) -> Result<Vec<UserPoolDescriptionType>, String> 
    {
        let req = ListUserPoolsRequest { max_results: 100, next_token: None };
        match self.idp.list_user_pools(req).await {
            Ok(res) => Ok(res.user_pools.unwrap()),
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn signup_user(&self, username: String, password: String, attributes: Option<Vec<AttributeType>>) 
        ->  RusotoResult<CgSignupRes, SignUpError>
    {
        let req = SignUpRequest {
            client_id: get_client_id(),
            secret_hash: Some(get_client_secret()),
            username, password, user_attributes: attributes,
            validation_data: None,
            client_metadata: None,
            user_context_data: None,
            ..Default::default()
        };
        match self.idp.sign_up(req).await {
            Ok(resp) => Ok(CgSignupRes::from(resp)),
            Err(err) => Err(err)
        }
    }
}

fn get_provider() -> CognitoProvider {
    let region = Region::UsWest2;
    let provider = CognitoProvider::builder()
         .region(region.clone())
         .build();
    provider
}

async fn register_user_cognito() {
    let client = get_idp_client();
    let create_identity_input = CreateIdentityPoolInput {
        identity_pool_name: "divisid".to_string(),
        developer_provider_name: Some("test".to_string()),
        ..Default::default()
    };

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn list_user_pools_ok() {
        let client = CognitoClient::new();
        let pools =  client.list_user_pools().await;
        assert!(pools.is_ok()) 
    }
}
