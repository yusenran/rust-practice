
mod greet;

use greet::sub_greet;

fn main() {
    println!("{}", sub_greet::hello_to_world());
}