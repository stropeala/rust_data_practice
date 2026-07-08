mod clients;

use clients::add_client;

fn main() {
    println!("Hello");

    add_client("salut", "salut", "salut", "salut").expect("penis");
    add_client("salut2", "salut2", "salut2", "salut2").expect("penis");
}
