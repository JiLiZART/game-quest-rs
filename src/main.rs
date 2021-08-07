mod world;
mod player;
mod place;
mod action;
mod door;
mod place_item;
mod game_item;
mod action_look_around;

use std::io;
use world::*;
use std::rc::Rc;

fn main() {

    let stdin = io::stdin();
    let input = &mut String::new();
    let mut world: Option<Rc<World>> = None;

    match world {
        Some(w) => {
            loop {
                input.clear();
                stdin.read_line(input);

                if input.len() > 0 {
                    let args: Vec<&str> = input.trim().split(" ").collect();

                    println!("{}", w.handle_action(args[0], args[1..].to_owned()))
                }
            }
        },
        None => {
            println!("Hello, whats your name?");

            loop {
                input.clear();
                stdin.read_line(input);

                if input.len() > 0 {
                    world = Some(World::new(input.trim()))
                }
            }
        }
    }
}
