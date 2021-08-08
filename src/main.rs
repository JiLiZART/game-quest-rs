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
    let world = World::new("player");
    let input = &mut String::new();

    println!("Привет, ты проснулся!");
    println!("доступные команды:");
    for action in world.actions.iter() {
        println!("{}", action.get_name())
    }

    loop {
        input.clear();
        stdin.read_line(input);

        if input.len() > 0 {
            let args: Vec<&str> = input.trim().split(" ").collect();

            println!("{}", world.handle_action(args[0], args[1..].to_owned()))
        }
    }
}
