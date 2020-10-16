use uuid::Uuid;
use dynomite::{
    Item, FromAttributes,
};

#[derive(Default, Item, Debug, Clone)]
pub struct DynamoUser {
    #[dynomite(partition_key, rename="uid")]
    pub uid: Uuid,
    #[dynomite(rename = "username", default)]
    pub username: String,
    #[dynomite(rename = "email", default)]
    pub email: String,
    #[dynomite(rename = "created_at", default)]
    pub created_at: String,
    #[dynomite(rename = "modified_at", default)]
    pub modified_at: String,
    #[dynomite(rename = "confirmed", default)]
    pub confirmed: bool,
    #[dynomite(rename = "", default)]
    pub enamed: bool,
}

//impl From<GetItemResponse> for DynamoUser 
