struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
/*    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotherExample@example.com"),
        sign_in_count: user1.sign_in_count,
    };*/
    // another simple way
      let user2 = User {
          email: String::from("anotherSimple@exmaple.com"),
          ..user1 // The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance
      };
}

