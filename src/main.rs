mod action;
mod action_look_around;
mod door;
mod game_item;
mod place;
mod place_item;
mod player;
mod world;

use std::io;
use std::rc::Rc;
use world::*;

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
        println!("[{}][0] > ", world.player.name);
        input.clear();
        stdin.read_line(input);

        if input.len() > 0 {
            let args: Vec<&str> = input.trim().split(" ").collect();

            println!("{}", world.handle_action(args[0], args[1..].to_owned()))
        }
    }
}
