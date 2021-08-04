use crate::place::Place;
use crate::game_item::GameItem;


pub struct Player<'a> {
    pub name: Box<&'a str>,
    pub place: Option<&'a Place<'a>>,
    pub has_inventory: bool,
    inventory: Option<Box<Vec<GameItem>>>
}

impl Player<'_> {
    pub fn new() -> Self {
        Self {
            name: Box::new(""),
            place: None,
            has_inventory: false,
            inventory: None,
        }
    }
}
