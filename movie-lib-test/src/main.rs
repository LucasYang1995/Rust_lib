extern crate movies_lib;
use movies_lib::movies::play;

fn main() {
    println!("inside main of test");
    play("SongKookRyol".to_string())
}