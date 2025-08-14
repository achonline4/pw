/* Individual elements are not mutable
 * Either the whole struct is mutable or none of it. */
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    // NOTE: that we are not declaring variable names with 'let' here. We are just
    // specifying the struct field names, that is required when we declare struct instances.
} // NOTE: we don't need a terminating ';' like in C.
fn main() {
    println!("Using structs");
    let mut user_ac = User {
        active: true,
        username: String::from("ac"),
        email: String::from("ac@gmail.com"),
        sign_in_count: 1,
    };  /* here we use a semi colon to terminate the struct definition */
    println!("struct {}", user_ac.username);
}
