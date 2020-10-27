use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rusoto_cognito_idp::{ AttributeType, NewDeviceMetadataType,
    AdminInitiateAuthError, AdminInitiateAuthRequest, AdminInitiateAuthResponse,
    AdminCreateUserResponse,
    SignUpError, SignUpResponse, SignUpRequest, AdminGetUserResponse,
    AuthenticationResultType, CodeDeliveryDetailsType, UserType
};

#[derive(Default, Serialize, Deserialize)]
pub struct CgSignupRes {
    pub uid: String,
    pub confirmed: bool,
    pub code_details: Option<CodeDeliveryDetailsType>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CodeDetails { pub dest: String, pub medium: String, pub attrib: String }

#[derive(Default, Serialize, Deserialize)]
pub struct CgUserSignup {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CgUserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CgUser {
    pub username: String,
    pub enabled: bool,
    pub attributes: Option<Vec<AttributeType>>,
    pub created_at: Option<f64>,
    pub modified_at: Option<f64>,
    pub status: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CgAuthRes {
    pub access_token: String,
    pub expires_in: i64,
    pub id_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub device: CgDeviceMeta,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CgDeviceMeta {
    pub device_group_key: String,
    pub device_key: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Challenge {
    pub name: String, 
    pub params: HashMap<String, String>,
}

impl From<AdminGetUserResponse> for CgUser  {
    fn from(user: AdminGetUserResponse) -> Self { 
        Self {
            username: user.username,
            attributes: user.user_attributes,
            status: user.user_status,
            enabled: user.enabled.unwrap(),
            created_at: user.user_create_date,
            modified_at: user.user_last_modified_date,
        }
    }
}

impl From<UserType> for CgUser  {
    fn from(user: UserType) -> Self { 
        Self {
            username: user.username.unwrap(),
            enabled: user.enabled.unwrap(),
            attributes: user.attributes,
            status: user.user_status,
            created_at: user.user_create_date,
            modified_at: user.user_last_modified_date,
        }
    }
}

impl From<AuthenticationResultType> for CgAuthRes {
    fn from(res: AuthenticationResultType) -> Self {
        Self {
            refresh_token: res.refresh_token.unwrap(),
            access_token: res.access_token.unwrap(),
            expires_in: res.expires_in.unwrap(),
            id_token: res.id_token.unwrap(),
            device: CgDeviceMeta::from(res.new_device_metadata),
            token_type: res.token_type.unwrap(),
        }
    }
}

impl From<Option<NewDeviceMetadataType>> for CgDeviceMeta  {
    fn from(data: Option<NewDeviceMetadataType>) -> Self { 
        let data = data.expect("Could not get device metadata");
        Self { 
            device_key: data.device_key.unwrap(),
            device_group_key: data.device_group_key.unwrap(),
        }
    }
}


impl From<SignUpResponse> for CgSignupRes {
    fn from(res: SignUpResponse) -> Self {
        Self {
            uid: res.user_sub,
            confirmed: res.user_confirmed,
            code_details: res.code_delivery_details,
        }
    }

}
