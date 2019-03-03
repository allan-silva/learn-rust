struct User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let user_name = String::from("Allan");
    let email = String::from("allan.tavares@allantavares.com.br");

    let mut user = build_user(&user_name, &email);
    user.active = false;

    println!("Name: {}, Email: {}, Active: {}",user_name, user.email, user.active);
}

fn build_user(user_name: &str, email: &str) -> User {
    User {
        user_name: String::from(user_name),
        email: String::from(email),
        sign_in_count: 1,
        active: true
    }
}
