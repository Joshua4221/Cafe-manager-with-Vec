pub enum AuthMenu {
    Admin,
    Client,
}

impl AuthMenu {
    pub fn select_auth_type(input: &str) -> Option<AuthMenu> {
        match input {
            "1" => Some(Self::Admin),
            "2" => Some(Self::Client),
            _ => None,
        }
    }

    pub fn show_auth_type() {
        println!("== Select What Type of User Are You ==");
        println!("1. Admin");
        println!("2. Client");
        println!("==");
        println!("Please enter your selection:");
    }
}
