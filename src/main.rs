mod world;
mod player;
mod place;
mod action;
mod door;
mod place_item;
mod game_item;

use std::io;
use world::*;

fn main() {
    println!("Hello, world!");

    let stdin = io::stdin();
    let input = &mut String::new();
    let world = World::new();

    loop {
        input.clear();
        stdin.read_line(input);

        if input.len() > 0 {
            println!("{}", world.handle_action(input.trim()))
        }
    }
}
