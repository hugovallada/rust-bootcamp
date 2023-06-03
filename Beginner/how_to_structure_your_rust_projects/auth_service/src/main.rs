use auth_service::Credentials;

fn main() {
    let credentials = Credentials::new("Hugo", "123");
    auth_service::authenticate(credentials)
}
