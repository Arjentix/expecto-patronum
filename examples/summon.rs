use expecto_patronum::prelude::*;

fn main() {
    println!("Cast a spell!");

    let mut spell = String::new();
    std::io::stdin()
        .read_line(&mut spell)
        .expecto_patronum("Failed to read line");

    Option::<()>::None.expecto_patronum(&spell);
}
