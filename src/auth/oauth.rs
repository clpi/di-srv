// use actix_web_httpauth::{
//     extractors::{AuthExtractor, AuthExtractorConfig},
// }
use tokio::runtime::Runtime;
use oauth2::{
    basic::{BasicClient, BasicTokenType, BasicTokenResponse,},
    url::Url, reqwest::http_client,
    AuthUrl, Client, HttpResponse, ClientId, ClientSecret, TokenUrl,
    RedirectUrl, CsrfToken, Scope, PkceCodeChallenge, AuthorizationCode,
};
use std::fmt::Debug;

#[derive(Debug)]
pub enum AuthType {
    AuthGet,
    AuthPost(String),
    Nothing,
}

pub struct OAuthClient {
    client: BasicClient,
}


/*
impl OAuthClient {

    pub async fn sample_auth() -> Self {
        let redirect = RedirectUrl::new("http://localhost:7777/authorized".into())
            .unwrap();
        let client = BasicClient::new(
            ClientId::new("client_id".to_string()),
            Some(ClientSecret::new("client_secret".into())),
            AuthUrl::new("https://localhost:7777/authorize".into()).unwrap(),
            Some(TokenUrl::new("https://localhost:7777/auth/token".into()).unwrap()),
        )
            .set_redirect_url(redirect);
        let (pkce_chal, pkce_veri) = PkceCodeChallenge::new_random_sha256();
        let (auth, csrf) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read".into()))
            .add_scope(Scope::new("write".into()))
            .set_pkce_challenge(pkce_chal)
            .url();
        let mut runtime = Runtime::new().unwrap();
        let token_res = runtime.block_on(
            client
                .exchange_code(AuthorizationCode::new("code".into()))
                .set_pkce_verifier(pkce_veri)
                .request(http_client).unwrap()
            );
        Self { client }
    }

}
*/
