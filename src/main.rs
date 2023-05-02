
fn main() {
    use nutrition_rust::user::{User, Gender};

    let user: User = User::new( "Neil".to_string(),"Colin".to_string(), 33, Gender::Male );
    user.print_name();
    println!("EMAIL:  {}", user.get_email());
}
