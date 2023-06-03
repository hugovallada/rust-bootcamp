pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn new(username: &str, password: &str) -> Credentials {
        Credentials {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
}
