use rusoto_core::Region;
use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};
use rusoto_cognito_idp::{ AttributeType, NewDeviceMetadataType,
    AdminGetUserRequest, AdminGetUserResponse, AdminGetUserError, AuthenticationResultType,
    AdminInitiateAuthError, AdminInitiateAuthRequest, AdminInitiateAuthResponse,
    AdminCreateUserError, AdminCreateUserRequest, AdminCreateUserResponse,
    CognitoIdentityProviderClient, CognitoIdentityProvider,
    SignUpRequest, ConfirmSignUpRequest, SignUpError, SignUpResponse,
    AdminConfirmSignUpResponse, AdminConfirmSignUpError,
    ListUsersRequest, ListUsersResponse, GlobalSignOutRequest,
    AdminConfirmSignUpRequest, InitiateAuthRequest,
    AdminUserGlobalSignOutRequest, AdminUserGlobalSignOutError, AdminUserGlobalSignOutResponse
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

#[derive(Default, Serialize, Deserialize)]
pub struct CognitoUser {
    pub username: String,
    pub attributes: Option<Vec<AttributeType>>,
    pub created_at: Option<f64>,
    pub modified_at: Option<f64>,
    pub status: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CognitoAuthResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub id_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub device: CognitoDeviceData,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CognitoDeviceData {
    pub device_group_key: String,
    pub device_key: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Challenge {
    pub name: String, 
    pub params: HashMap<String, String>,
}

impl CognitoUser {
    
}

impl From<AdminGetUserResponse> for CognitoUser  {
    fn from(user: AdminGetUserResponse) -> Self { 
        Self {
            username: user.username,
            attributes: user.user_attributes,
            status: user.user_status,
            created_at: user.user_create_date,
            modified_at: user.user_last_modified_date,
        }
    }
}

impl From<AuthenticationResultType> for CognitoAuthResponse {
    fn from(res: AuthenticationResultType) -> Self {
        CognitoAuthResponse {
            refresh_token: res.refresh_token.unwrap(),
            access_token: res.access_token.unwrap(),
            expires_in: res.expires_in.unwrap(),
            id_token: res.id_token.unwrap(),
            device: CognitoDeviceData::from(res.new_device_metadata),
            token_type: res.token_type.unwrap(),
        }
    }
}

impl From<Option<NewDeviceMetadataType>> for CognitoDeviceData  {
    fn from(data: Option<NewDeviceMetadataType>) -> Self { 
        let data = data.expect("Could not get device metadata");
        Self { 
            device_key: data.device_key.unwrap(),
            device_group_key: data.device_group_key.unwrap(),
        }
    }
}

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
        username: String) ->  Result<CognitoUser, AdminGetUserError> 
    {
        let req = AdminGetUserRequest {
            username, user_pool_id: get_user_pool_id(),
        };
        match self.idp.admin_get_user(req).await {
            Ok(user) =>  Ok(CognitoUser::from(user)),
            Err(_err) => { Err(AdminGetUserError::UserNotFound("Not found".to_string())) }
        }

    }

    pub async fn login_user(&self, username: String, password: String) 
        ->  Result<CognitoAuthResponse, AdminInitiateAuthError> 
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
                Some(res) => {
                    return Ok(CognitoAuthResponse::from(res));
                },
                None => {
                    return Err(AdminInitiateAuthError::NotAuthorized("Couldnt authorize".to_string())); 
                }
            },
            Err(err) => Err(AdminInitiateAuthError::NotAuthorized("Couldnt authorize".to_string()))
                
        }
    }

    pub async fn create_user(&self, username: String) -> () {}

    pub async fn signout_user(&self, username: String) -> () {}

    pub async fn signup_user(&self, username: String, password: String, attributes: Option<Vec<AttributeType>>) 
//        ->  Result<CognitoAuthResponse, SignUpError> 
        -> ()
    {
        let req = SignUpRequest {
            analytics_metadata: None,
            client_id: get_client_id(),
            secret_hash: Some(get_client_secret()),
            username, password, user_attributes: attributes,
            validation_data: None,
            client_metadata: None,
            user_context_data: None,
            ..Default::default()
        };
        match self.idp.sign_up(req).await {
            Ok(resp) => { 
                if resp.user_confirmed {
                    match resp.code_delivery_details {
                        Some(code) => { ()
                            //( dest: code.destination.unwrap()
                                //code.attribute_name.unwrap(),
                                //code.delivery_medium.unwrap() );
                            //}
                        }
                        None => {}
                    }
                
                } else {

                }
            }
            //Err(err) => Err(SignUpError::UsernameExists("".to_string()))
            Err(err) => (),
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
