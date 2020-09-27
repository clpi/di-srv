use common::models::User;

#[test]
fn can_signup_user() {
    let user = User::new(
        String::from("test@c.com"),
        String::from("test"),
        String::from("pass"),
    );
}
