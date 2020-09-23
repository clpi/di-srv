use actix_web::test;

#[cfg(test)]
pub mod signup {

    use super::*;

    #[actix_rt::test]
    async fn signup_valid_user_works() {
        
    }

    #[actix_rt::test]
    async fn signup_invalid_email_fails() {

    }

    #[actix_rt::test]
    async fn signup_invalid_username_fails() {

    }

    #[actix_rt::test]
    async fn signup_invalid_password_fails() {

    }

}

#[cfg(test)]
pub mod login {

    use super::*;

    #[actix_rt::test]
    async fn login_invalid_user_fails() {

    }

    #[actix_rt::test]
    async fn login_missing_user_fails() {

    }

    #[actix_rt::test]
    async fn login_valid_user_works() {
        
    }

}
