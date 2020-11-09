pub mod support;
pub mod oauth;
pub mod jwt;

#[derive(Debug, Default)]
pub(crate) struct State {
    pub token: Option<String>,
    pub refresh: Option<String>,
    pub until: Option<i64>,
}

impl State {

    pub fn _new(
        token: Option<String>, refresh: Option<String>, until: Option<i64>) -> Self { Self { token, refresh, until }
    }
}

