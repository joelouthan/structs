struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("joe@theologic.us"),
        String::from("joseph"),
    );

    let user2 = User {
        email: String::from("joelouthan@proton.me"),
        ..user1
    };
}