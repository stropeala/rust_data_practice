mod clients;

use clients::add_client;

fn main() {
    println!("Hello");

    add_client("salut", "sunt", "un", "test").expect("Could not add new Client!");
}
